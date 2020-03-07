use seed::browser::service::fetch;
use seed::{*, prelude::*};
use getrandom;
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

async fn fetch_questions() -> Result<Msg, Msg> {
    fetch::Request::new("/questions.json")
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
    
    match model.state {
        State::Started | State::Loading | State::Done(_) => {
            match msg {
                Msg::Start => {
                    if model.questions.len() > 0 {
                        model.state = State::Playing(
                            PlayState{
                                score:0,
                                tries:0,
                                current_question: get_random(model.questions.len()),
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
                        ps.current_question = get_random(model.questions.len());
                        ps.state = AnsweringQuestionState::NotAnswered;
                    }
                },
                _ => ()
            }
        },
    }
}

fn question_view(question: &Question, state: &AnsweringQuestionState) -> Node<Msg> {
    match state {
        AnsweringQuestionState::NotAnswered => {
            div![
                h3![question.caption],
                img![attrs!{
                    At::Src => question.image_url
                }],
                button![simple_ev(Ev::Click, Msg::AnswerTrue), " NOT Disney Vacation!" ],
                button![simple_ev(Ev::Click, Msg::AnswerFalse), " Disney Vacation!" ]
            ]
        },
        AnsweringQuestionState::Correct => {
            div![
                h3!["Correct!!"],
                button![simple_ev(Ev::Click, Msg::NextQuestion), "Ask Me another" ],
            ]
        },
        AnsweringQuestionState::Incorrect => {
            div![
                h3!["Incorrect!! :("],
                button![simple_ev(Ev::Click, Msg::NextQuestion), "Ask Me another" ],
            ]          
        }
    }
    

}

fn view(model: &Model) -> impl View<Msg> {
    match &model.state {
        State::Started => div![
                h1![ "Disney Vacation / Not Disney Vacation"],
                h3![ "The game where you try and guess if ridiculous wikihow captions are real" ],
                button![simple_ev(Ev::Click, Msg::Start), "Start!" ],
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
                h3!["Game Over!"],
                h5![result_message],
                button![simple_ev(Ev::Click, Msg::Start), "Why not another?" ],
            ]
            
       }
    }

}

#[cfg(target_arch = "wasm32")]
fn get_random(max: usize) -> usize {
    let mut my_bytes: [u8; 4] = [0; 4];
    if let Ok(_) = getrandom::getrandom(&mut my_bytes) {
        return usize::from_be_bytes(my_bytes) % max;
    }
    0
}

#[cfg(not(target_arch = "wasm32"))]
fn get_random(max: usize) -> usize {
    let mut my_bytes: [u8; 8] = [0; 8];
    if let Ok(_) = getrandom::getrandom(&mut my_bytes) {
        return usize::from_be_bytes(my_bytes) % max;
    }
    0
}


#[wasm_bindgen(start)]
pub fn render() {
    App::builder(update, view)
        .after_mount(after_mount)
        .build_and_start();
}