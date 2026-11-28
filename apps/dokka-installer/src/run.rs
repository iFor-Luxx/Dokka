//! Bucle de la ventana: dibuja la isla y atiende sus acciones.
//!
//! Crea la ventana centrada, le pone la foto de fondo y repinta la escena
//! solo si cambió lo dibujado. El trabajo corre en `worker` y su avance
//! llega como `Refresh`.

use std::error::Error;
use std::sync::mpsc::{Receiver, Sender, channel};

use dokka_setup::{Setup, SetupAction};
use dokka_theme::{installer as measures, typography};
use dokka_win_composition::{InstallerPhoto, InstallerRenderer};
use dokka_win_surface::{InstallerEvent, InstallerWindow};

use crate::worker::{WorkerMsg, spawn_install, spawn_uninstall};

/// Aviso propio para recoger el canal del hilo de trabajo.
const WAKE_MESSAGE: u32 = 0x8101;

/// Foto de fondo embebida en el binario.
static BEACH: &[u8] = include_bytes!("../../assets/installer/playa.jpg");

/// Todo lo que la ventana necesita para atender un evento.
struct Ui {
    renderer: InstallerRenderer,
    theme: dokka_theme::Theme,
    setup: Setup,
    painted: Option<dokka_setup::PaintedSetup>,
    sender: Sender<WorkerMsg>,
    receiver: Receiver<WorkerMsg>,
}

/// Arranca la ventana y la atiende hasta que se cierre.
pub fn run() -> Result<(), Box<dyn Error>> {
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
    let renderer = InstallerRenderer::attach(
        window.raw(),
        window.scale(),
        typography::FONT_FAMILY,
        dokka_icons::LUCIDE_FONT_BYTES,
        &photo,
    )?;
    let (sender, receiver) = channel::<WorkerMsg>();
    let mut ui = Ui {
        renderer,
        theme,
        setup: Setup::new(uninstall),
        painted: None,
        sender,
        receiver,
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
            ui.setup.pointer_moved(x, y);
            draw(ui);
        }
        InstallerEvent::PointerLeft => {
            ui.setup.pointer_left();
            draw(ui);
        }
        InstallerEvent::Pressed { x, y } => {
            ui.setup.press(x, y);
            if ui.setup.drag_zone(x, y) {
                window.begin_drag();
            }
            draw(ui);
        }
        InstallerEvent::Clicked { x, y } => {
            let action = ui.setup.release(x, y);
            draw(ui);
            act(window, action, ui);
        }
        InstallerEvent::Dismissed => {
            if !ui.setup.is_working() {
                window.close();
            }
        }
        InstallerEvent::Tick => {}
        InstallerEvent::Refresh => {
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
            window.raw(),
            WAKE_MESSAGE,
        );
    } else {
        spawn_install(
            ui.setup.start_with_windows(),
            ui.setup.desktop_shortcut(),
            ui.sender.clone(),
            window.raw(),
            WAKE_MESSAGE,
        );
    }
}
