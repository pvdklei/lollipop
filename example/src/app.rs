use lollipop::{console_log, div, h1, input, Action, Html, Widget};

struct State {
    count: usize,
    input: String,
}
pub enum Msg {
    IncreaseCount(),
    InputChanged(String),
}

type Props = ();

pub fn app() -> Widget {
    Widget::new("app", (), state, update, view)
}

fn state() -> State {
    State {
        count: 0,
        input: "".to_string(),
    }
}

fn update(state: &mut State, _props: &Props, msg: Msg) -> Action<Msg> {
    match msg {
        Msg::IncreaseCount() => {
            state.count += 1;
            Action::Diff
        }
        Msg::InputChanged(v) => {
            console_log!("{}", v);
            state.input = v;
            Action::Diff
        }
    }
}

fn view(_state: &State, _props: &Props) -> Html<Msg> {
    div()
        .on("click", Msg::IncreaseCount)
        .child(h1().child("Yoooo"))
        .child(
            input()
                .on_value("change", Msg::InputChanged)
                .attr("label", "cdhks"),
        )
}
