use seed::browser::service::fetch;
use seed::{*, prelude::*};
use rand::prelude::*;
use serde::Deserialize;

// Game constants
static NUM_TRIES: usize = 10;


struct Model {
    state: State,
    questions: Vec<Question>,
}

#[derive(Debug)]
enum State {
    Started,
    Loading,
    Playing(PlayState),
    Done(usize),
}

#[derive(Debug)]
struct PlayState {
    score: usize,
    tries: usize,
    current_question: usize,
    state: AnsweringQuestionState,
}

#[derive(Debug)]
enum AnsweringQuestionState {
    NotAnswered,
    Correct,
    Incorrect,
}

#[derive(Debug, Deserialize, Clone)]
struct Question {
    is_real: bool,
    caption: String,
    source_url: String,
    reddit_url: String,
    image_url: String,
}

#[derive(Debug, Clone)]
enum Msg {
    Start,
    FetchedQuestions(fetch::ResponseDataResult<Vec<Question>>),
    AnswerTrue,
    AnswerFalse,
    NextQuestion,
}

impl Default for Model {
    fn default() -> Self {
        Self {
            state: State::Started,
            questions: vec![],
        }
    }
}

#[cfg(debug_assertions)]
async fn fetch_questions() -> Result<Msg, Msg> {
    fetch::Request::new("/questions.json")
        .method(fetch::Method::Get)
        .fetch_json_data(Msg::FetchedQuestions)
        .await
}

#[cfg(not(debug_assertions))]
async fn fetch_questions() -> Result<Msg, Msg> {
    fetch::Request::new("/dv-or-naw/questions.json")
        .method(fetch::Method::Get)
        .fetch_json_data(Msg::FetchedQuestions)
        .await
}

fn after_mount(_url: Url, orders: &mut impl Orders<Msg>)  -> AfterMount<Model> {
    orders.perform_cmd(fetch_questions());
    let model = Default::default();
    AfterMount::new(model).url_handling(UrlHandling::None)
}

fn update(msg: Msg, model: &mut Model, _orders: &mut impl Orders<Msg>) {
    let mut rng = thread_rng();
    match model.state {
        State::Started | State::Loading | State::Done(_) => {
            match msg {
                Msg::Start => {
                    if model.questions.len() > 0 {
                        model.state = State::Playing(
                            PlayState{
                                score:0,
                                tries:0,
                                current_question: rng.gen_range(0, model.questions.len()),
                                state:AnsweringQuestionState::NotAnswered,
                        });
                    } else {
                        model.state = State::Loading;
                    }

                },
                Msg::FetchedQuestions(Ok(questions)) => {
                    model.questions = questions;
                },
                Msg::FetchedQuestions(Err(reason)) => {
                    error!("Request Failed!", reason);
                },
                _ => {()},
            }
        },
        State::Playing(ref mut ps) => {
            match msg {
                Msg::AnswerTrue => {
                    ps.tries += 1;
                    if model.questions[ps.current_question].is_real {
                        // Correct!
                        ps.score += 1;
                        ps.state = AnsweringQuestionState::Correct;
                    } else {
                        ps.state = AnsweringQuestionState::Incorrect;
                    }
                },
                Msg::AnswerFalse => {
                    ps.tries += 1;
                    if !model.questions[ps.current_question].is_real {
                        // Correct!
                        ps.score += 1;
                        ps.state = AnsweringQuestionState::Correct;
                    } else {
                        ps.state = AnsweringQuestionState::Incorrect;
                    }                   
                },
                Msg::NextQuestion => {
                    if ps.tries >= NUM_TRIES {
                        model.state = State::Done(ps.score);
                    } else {
                        ps.current_question = rng.gen_range(0, model.questions.len());
                        ps.state = AnsweringQuestionState::NotAnswered;
                    }
                },
                _ => ()
            }
        },
    }
}

fn link(href: &str, text: &str) -> Node<Msg> {
    a![
        class!["underline", "text-blue-400"],
        attrs!{At::Href => href},
        text
    ]
}

fn question_view(question: &Question, state: &AnsweringQuestionState) -> Node<Msg> {
    div![
        class!["container-md", "mx-auto", "content-center", "mt-4"],
        h3![
            class!["text-center", "text-2xl"],
            question.caption],
        div![
            class!["container-sm", "m-8", "max-w-xl", "overflow-hidden"],
            img![
            class!["max-w-xl"],
            style!{"margin-bottom" => "-30px", "overflow" => "hidden"},
            attrs!{
                At::Src => question.image_url
            }],
            svg![
            style!{"width" => "1000px", "height" => "30px"},
            rect![attrs!{
                At::X => "0",
                At::Y => "0",
                At::Width => "1000",
                At::Height => "30",
                At::Stroke => "white",
                At::Fill => "white"
            }]]
        ],
        match state {
            AnsweringQuestionState::NotAnswered => {
                div![
                    class!["flex", "flex-row", "justify-center"],
                    button![class!["bg-blue-500", "hover:bg-blue-700", "text-white", "font-bold", "py-2", "px-4", "rounded", "m-2"],
                        simple_ev(Ev::Click, Msg::AnswerTrue), " NOT Disney Vacation!" ],
                    button![class!["bg-blue-500", "hover:bg-blue-700", "text-white", "font-bold", "py-2", "px-4", "rounded", "m-2"],
                        simple_ev(Ev::Click, Msg::AnswerFalse), " Disney Vacation!" ]
                ]
            },
            AnsweringQuestionState::Correct => {
                div![
                    class!["container-md", "max-w-sm", "mx-auto", "content-center", "mt-4", "flex", "flex-col", "items-center"],
                    h3![
                        class!["text-center", "text-xl", "bg-green-200"],
                        "Correct!!"],
                    button![class!["text-center", "bg-blue-500", "hover:bg-blue-700", "text-white", "font-bold", "py-2", "px-4", "rounded", "m-2"],
                        simple_ev(Ev::Click, Msg::NextQuestion), "Ask Me another" ],
                    p![
                        class!["text-center"],
                        link(&question.reddit_url, "reddit link")],
                    p![
                        class!["text-center"],
                        link(&question.source_url, "wikihow link")],

                ]
            },
            AnsweringQuestionState::Incorrect => {
                div![
                    class!["container-md", "max-w-sm" "mx-auto", "content-center", "mt-4", "flex", "flex-col", "items-center"],
                    h3![
                        class!["text-center", "text-xl", "bg-red-300"],
                        "Incorrect!! :("],
                    button![class!["text-center", "bg-blue-500", "hover:bg-blue-700", "text-white", "font-bold", "py-2", "px-4", "rounded", "m-2"],
                        simple_ev(Ev::Click, Msg::NextQuestion), "Ask Me another" ],
                    p![
                        class!["text-center"],
                        link(&question.reddit_url, "reddit link")],
                    p![
                        class!["text-center"],
                        link(&question.source_url, "wikihow link")],

                ]          
            }
        }
    ]
    

}

fn view(model: &Model) -> impl View<Msg> {
    let content = match &model.state {
        State::Started => 
            div![
                class!["container-md", "mx-auto", "content-center", "mt-4"],
                div![
                    class!["flex", "flex-col", "justify-center", "m-4", "max-w-xl"],
                    h1![
                        class!["text-2xl", "text-center"],
                        "Disney Vacation / Not Disney Vacation"],
                    h3![
                        class!["text-xl", "text-center"],
                        "The game where you try and guess if ridiculous wikihow captions are real" ],
                    h4![
                        class!["text-lg", "text-center"],
                        "How to play:"],
                    p![
                        class!["text-center"],
                        "Each round you'll be given a caption and an image from wikihow. 
                        If you think it is FAKE click Disney Vacation. If you think it is REAL click NOT 
                        Disney Vacation. Answer 10 questions and we will give you a score! Good luck!"],
                    button![class!["bg-blue-500", "hover:bg-blue-700", "text-white", "font-bold", "py-2", "px-4", "rounded", "m-2"],
                        simple_ev(Ev::Click, Msg::Start), "Start!" ],
                ]
            ],
        State::Loading => h3!["Loading...."],
        State::Playing(ps) => {
            let ref question: Question = model.questions[ps.current_question];

            question_view(question, &ps.state)
        },
       State::Done (score)=> {
            let result_message = match score {
                0 ..= 3 => format!("You scored {}/10. Too bad 😰 try again next time!", score),
                4 ..= 7 => format!("You scored {}/10. Not bad! 🤔👍", score),
                8 ..= 9 => format!("You scored {}/10. Heyyyyy, that's pretty good!!", score),
                10 => String::from("You scored 10/10! Perfect score!"),
                _ => format!("Um, you scored {}/10.... How about filing an issue on Github?", score)
            };
            
            div![
                class!["container-md", "mx-auto", "flex", "flex-col", "justify-center", "mt-4"],
                h3![
                    class!["text-xl", "text-center"],
                    "Game Over!"
                    ],
                h5![
                    class!["text-lg", "text-center"],
                    result_message],
                button![class!["bg-blue-500", "hover:bg-blue-700", "text-white", "font-bold", "py-2", "px-4", "rounded", "m-2"],
                    simple_ev(Ev::Click, Msg::Start), "Why not another?" ],
            ]
            
       }
    };
    div![
        class!["container-md flex mx-auto"],
        content
    ]

}

#[wasm_bindgen(start)]
pub fn render() {
    App::builder(update, view)
        .after_mount(after_mount)
        .build_and_start();
}