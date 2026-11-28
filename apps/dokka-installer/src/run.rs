//! Bucle de la ventana: dibuja la isla y atiende sus acciones.
//!
//! Crea la ventana centrada, le pone la foto de fondo y repinta la escena
//! solo si cambió lo dibujado. El trabajo corre en `worker` y su avance
//! llega como `Refresh`.

use std::error::Error;
use std::sync::mpsc::{Receiver, Sender, channel};

use dokka_setup::{Setup, SetupAction};
use dokka_theme::{installer as measures, typography};
use dokka_win_composition::{InstallerEntryData, InstallerPhoto, InstallerRenderer};
use dokka_win_surface::{InstallerEvent, InstallerWindow};

use crate::worker::{WorkerMsg, spawn_install, spawn_uninstall};

/// Aviso propio para recoger el canal del hilo de trabajo.
const WAKE_MESSAGE: u32 = 0x8101;

/// Foto de fondo embebida en el binario.
static BEACH: &[u8] = include_bytes!("../../../assets/installer/playa.jpg");
/// Cielo, mar y arena sin montañas: base de la entrada.
static FONDO: &[u8] = include_bytes!("../../../assets/installer/fondo.jpg");
/// Montaña izquierda con transparencia real.
static IZQUIERDA: &[u8] = include_bytes!("../../../assets/installer/izquierda.png");
/// Montaña derecha con transparencia real.
static DERECHA: &[u8] = include_bytes!("../../../assets/installer/derecha.png");

/// Todo lo que la ventana necesita para atender un evento.
struct Ui {
    renderer: InstallerRenderer,
    theme: dokka_theme::Theme,
    setup: Setup,
    painted: Option<dokka_setup::PaintedSetup>,
    sender: Sender<WorkerMsg>,
    receiver: Receiver<WorkerMsg>,
    entry_start: std::time::Instant,
}

/// Arranca la ventana y la atiende hasta que se cierre.
pub(crate) fn run() -> Result<(), Box<dyn Error>> {
    let uninstall = std::env::args().any(|arg| arg == "--uninstall");
    let mut window = InstallerWindow::create()?;
    window.place(measures::WIDTH, measures::HEIGHT, measures::SHADOW_MARGIN)?;
    let theme = dokka_theme::light();
    let photo = InstallerPhoto {
        bytes: BEACH,
        fallback: theme.installer.fallback,
        width: measures::WIDTH,
        height: measures::HEIGHT,
        margin: measures::SHADOW_MARGIN,
        radius: measures::CORNER_RADIUS,
    };
    let entry = InstallerEntryData {
        fondo: FONDO,
        izquierda: IZQUIERDA,
        derecha: DERECHA,
        travel: measures::ENTRY_TRAVEL,
        blur_max: measures::ENTRY_BLUR_MAX,
        mountains_seconds: measures::ENTRY_MOUNTAINS_SECONDS,
        photo_fade_seconds: measures::ENTRY_PHOTO_FADE_SECONDS,
        content_delay_seconds: measures::ENTRY_CONTENT_DELAY_SECONDS,
        content_seconds: measures::ENTRY_CONTENT_SECONDS,
        rise: measures::ENTRY_CONTENT_RISE,
        ease_x1: measures::ENTRY_EASE_X1,
        ease_y1: measures::ENTRY_EASE_Y1,
        ease_x2: measures::ENTRY_EASE_X2,
        ease_y2: measures::ENTRY_EASE_Y2,
    };
    let renderer = InstallerRenderer::attach(
        window.raw(),
        window.scale(),
        typography::FONT_FAMILY,
        dokka_icons::LUCIDE_FONT_BYTES,
        &photo,
        &entry,
    )?;
    let (sender, receiver) = channel::<WorkerMsg>();
    let mut ui = Ui {
        renderer,
        theme,
        setup: Setup::new(uninstall),
        painted: None,
        sender,
        receiver,
        entry_start: std::time::Instant::now(),
    };
    draw(&mut ui);
    window.show()?;
    let result = window.run(WAKE_MESSAGE, |window, event| handle(window, event, &mut ui));
    result?;
    Ok(())
}

/// Dibuja si cambió lo dibujado.
fn draw(ui: &mut Ui) {
    let painted = ui.setup.painted();
    if ui.painted.as_ref() == Some(&painted) {
        return;
    }
    ui.painted = Some(painted);
    if ui
        .renderer
        .draw(&dokka_setup::setup_scene(&ui.theme, &mut ui.setup))
        .is_err()
    {
        ui.painted = None;
    }
}

/// Atiende un evento; devuelve si quiere seguir recibiendo fotogramas.
fn handle(window: &mut InstallerWindow, event: InstallerEvent, ui: &mut Ui) -> bool {
    match event {
        InstallerEvent::PointerMoved { x, y } => {
            retire_entry_if_done(ui);
            if entry_blocked(ui) {
                return false;
            }
            ui.setup.pointer_moved(x, y);
            draw(ui);
        }
        InstallerEvent::PointerLeft => {
            retire_entry_if_done(ui);
            if entry_blocked(ui) {
                return false;
            }
            ui.setup.pointer_left();
            draw(ui);
        }
        InstallerEvent::Pressed { x, y } => {
            retire_entry_if_done(ui);
            if entry_blocked(ui) {
                return false;
            }
            ui.setup.press(x, y);
            if ui.setup.drag_zone(x, y) {
                window.begin_drag();
            }
            draw(ui);
        }
        InstallerEvent::Clicked { x, y } => {
            retire_entry_if_done(ui);
            if entry_blocked(ui) {
                return false;
            }
            let action = ui.setup.release(x, y);
            draw(ui);
            act(window, action, ui);
        }
        InstallerEvent::Dismissed => {
            if !ui.setup.is_working() {
                window.close();
            } else if window.show().is_err() {
                // Windows ya la ocultó (Alt+F4): sin poder volver, sigue el trabajo.
                eprintln!("El instalador no pudo volver a mostrarse");
            }
        }
        // El segundo a segundo recupera avisos perdidos durante un arrastre.
        InstallerEvent::Tick | InstallerEvent::Refresh => {
            retire_entry_if_done(ui);
            while let Ok(message) = ui.receiver.try_recv() {
                match message {
                    WorkerMsg::Progress { step, fraction } => {
                        ui.setup.set_progress(step, fraction);
                    }
                    WorkerMsg::Done { detail } => {
                        if let Some(detail) = detail {
                            let title = if ui.setup.is_uninstall() {
                                "No se pudo desinstalar"
                            } else {
                                "No se pudo instalar"
                            };
                            ui.setup.fail(title, &detail);
                        } else {
                            ui.setup.finish();
                        }
                    }
                }
            }
            draw(ui);
        }
        InstallerEvent::Frame { seconds } => {
            if ui.setup.advance(seconds) {
                draw(ui);
                return true;
            }
        }
    }
    ui.setup.fade() < 1.0
}

/// Fin de la entrada: lo último entre el fundido de la foto y el contenido.
fn entry_total_seconds() -> f32 {
    let photo_end = measures::ENTRY_MOUNTAINS_SECONDS + measures::ENTRY_PHOTO_FADE_SECONDS;
    let content_end = measures::ENTRY_CONTENT_DELAY_SECONDS + measures::ENTRY_CONTENT_SECONDS;
    photo_end.max(content_end).max(0.0)
}

/// `true` mientras el contenido aún no está visible ni responde a clics.
fn entry_blocked(ui: &Ui) -> bool {
    ui.renderer.entry_running() && ui.entry_start.elapsed().as_secs_f32() < entry_total_seconds()
}

/// Retira fondo y montañas al pasar el tiempo total, sin efectos activos.
fn retire_entry_if_done(ui: &mut Ui) {
    if ui.renderer.entry_running()
        && ui.entry_start.elapsed().as_secs_f32() >= entry_total_seconds()
        && ui.renderer.finish_entry().is_err()
    {
        ui.painted = None;
    }
}

/// Ejecuta la acción que pide la isla.
fn act(window: &mut InstallerWindow, action: Option<SetupAction>, ui: &mut Ui) {
    match action {
        None => {}
        Some(SetupAction::Close) => {
            if !ui.setup.is_working() {
                window.close();
            }
        }
        Some(SetupAction::ShowOptions) => {
            ui.setup.show_options();
            draw(ui);
        }
        Some(SetupAction::BackWelcome) => {
            ui.setup.back_welcome();
            draw(ui);
        }
        Some(SetupAction::ToggleStart) => {
            ui.setup.toggle_start();
            draw(ui);
        }
        Some(SetupAction::ToggleShortcut) => {
            ui.setup.toggle_shortcut();
            draw(ui);
        }
        Some(SetupAction::OptionsDone) => {
            ui.setup.options_done();
            draw(ui);
        }
        Some(SetupAction::StartInstall) => start_work(window, ui, false),
        Some(SetupAction::StartUninstall) => start_work(window, ui, true),
        Some(SetupAction::ToggleRemove) => {
            ui.setup.toggle_remove();
            draw(ui);
        }
        Some(SetupAction::Retry) => {
            ui.setup.retry();
            draw(ui);
            launch_work(window, ui);
        }
        Some(SetupAction::Launch) => {
            use dokka_contracts::Installer;
            let installer = dokka_win_install::WindowsInstaller::new(crate::PAYLOAD);
            let _ = installer.launch_dokka();
            window.close();
        }
    }
}

/// Empieza el trabajo elegido si no hay otro en curso.
fn start_work(window: &mut InstallerWindow, ui: &mut Ui, uninstall: bool) {
    if ui.setup.is_working() {
        return;
    }
    if uninstall {
        ui.setup.start_uninstall();
    } else {
        ui.setup.start_install();
    }
    draw(ui);
    launch_work(window, ui);
}

/// Lanza el hilo del trabajo según la pantalla actual.
fn launch_work(window: &mut InstallerWindow, ui: &mut Ui) {
    if ui.setup.screen() == dokka_setup::Screen::Uninstalling {
        spawn_uninstall(
            ui.setup.remove_settings(),
            ui.sender.clone(),
            window.waker(WAKE_MESSAGE),
        );
    } else {
        spawn_install(
            ui.setup.start_with_windows(),
            ui.setup.desktop_shortcut(),
            ui.sender.clone(),
            window.waker(WAKE_MESSAGE),
        );
    }
}
