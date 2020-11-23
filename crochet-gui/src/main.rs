use kteach_core::engine::Engine;
use kteach_core::midi::midir::{MidiOutput, MidiOutputConnection};

use druid::{AppLauncher, PlatformError, Widget, WindowDesc};

use crochet::{AppHolder, Button, Column, Cx, DruidAppData, Row};
use druid::{Data, MenuDesc};

mod player;
use crate::player::{AddPlayer, Player};

struct FunctionBar {
    add_player: AddPlayer,
}

impl FunctionBar {
    fn new() -> Self {
        FunctionBar {
            add_player: AddPlayer::new(),
        }
    }

    fn run(&mut self, cx: &mut Cx, mut players: &mut Vec<Player>, mut engine: &mut Engine) {
        self.add_player.run(cx, &mut players, &mut engine);
        Button::new("fucntion2").build(cx);
        Button::new("fucntion3").build(cx);
        Button::new("fucntion4").build(cx);
        Button::new("fucntion5").build(cx);
    }
}

struct App {
    players: Vec<Player>,
    function_bar: FunctionBar,
    engine: Engine,
    midi_out: MidiOutputConnection,
}

impl App {
    fn new() -> Self {
        let engine = Engine::new();
        let midi_out = get_midi_out();
        App {
            players: Vec::new(),
            function_bar: FunctionBar::new(),
            engine,
            midi_out,
        }
    }

    fn run(&mut self, cx: &mut Cx) {
        Column::new().build(cx, |cx| {
            Row::new().build(cx, |cx| {
                self.function_bar
                    .run(cx, &mut self.players, &mut self.engine)
            });
            Row::new().build(cx, |cx| {
                for player in &mut self.players {
                    player.run(cx, &mut self.midi_out)
                }
            })
        })
    }
}

fn main() -> Result<(), PlatformError> {
    let main_window = WindowDesc::new(ui_builder).menu(make_menu(&MenuState::default()));
    let data = Default::default();

    AppLauncher::with_window(main_window)
        .use_simple_logger()
        .launch(data)
}

fn ui_builder() -> impl Widget<DruidAppData> {
    let mut app_logic = App::new();
    AppHolder::new(move |cx| app_logic.run(cx))
}

#[derive(Debug, Clone, Default, Data)]
struct MenuState {
    menu_count: usize,
    selected: usize,
    glow_hot: bool,
}

#[allow(unused_assignments)]
fn make_menu<T: Data>(_state: &MenuState) -> MenuDesc<T> {
    let mut base = MenuDesc::empty();
    #[cfg(target_os = "macos")]
    {
        base = druid::platform_menus::mac::menu_bar();
    }
    #[cfg(any(target_os = "windows", target_os = "linux"))]
    {
        base = base.append(druid::platform_menus::win::file::default());
    }
    base
}

pub fn get_midi_out() -> MidiOutputConnection {
    let midi_out = MidiOutput::new("").unwrap();

    let out_ports = midi_out.ports();
    let out_port = &out_ports[0];

    let mut conn_out = midi_out.connect(&out_port, "").unwrap();
    conn_out
}
