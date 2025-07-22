use color_eyre::owo_colors::OwoColorize;
use ratatui::buffer::Buffer;
use ratatui::layout::{Constraint, Rect};
use ratatui::prelude::{Layout, Text};
use ratatui::style::{Modifier, Styled};
use ratatui::text::Line;
use ratatui::widgets::{Widget, Wrap};
use ratatui::{
    layout::Alignment,
    style::{Color, Stylize},
    widgets::{Block, BorderType, Paragraph},
    Frame, Terminal,
};
use ratzilla::widgets::Hyperlink;
use ratzilla::{CanvasBackend, DomBackend, WebRenderer};
use std::iter::{once, zip};
use std::{io, rc::Rc};

fn main() -> io::Result<()> {
    let backend = DomBackend::new()?;
    let terminal = Terminal::new(backend)?;

    let state = Rc::new(App::default());

    let render_state = Rc::clone(&state);
    terminal.draw_web(move |frame| {
        render_state.render(frame);
    });

    Ok(())
}

#[derive(Default)]
struct App;

impl App {
    fn render(&self, frame: &mut Frame) {
        let [_, content] =
            Layout::vertical([Constraint::Length(3), Constraint::Fill(1)]).areas(frame.area());
        let block = Block::bordered()
            .title("Yew Jung")
            .title_alignment(Alignment::Center)
            .border_type(BorderType::Rounded)
            .bg(Color::Black);
        let inner_content = block.inner(content);
        frame.render_widget(block, content);
        let [summary, fun_stuff_area, exp_area] =
            Layout::vertical(Constraint::from_percentages([10, 20, 70])).areas(inner_content);
        frame.render_widget(Summary, summary);
        let experiences = Experiences {
            jobs: [
                Experience {
                    title: "Backend Engineer @ BigPay".to_owned(),
                    achievements: vec![
                        Line::from(vec![
                            "Led the architecture and implementation ".light_green().bold(),
                            "of credit and debit card top-ups, enabling seamless funding of user wallets at scale".into()
                        ]),
                        Line::from(vec![
                            "Designed and built ".light_green().bold(),
                            "a new in-person onboarding flow, significantly streamlining the signup process for face-to-face customer interactions".into()
                        ]),
                        Line::from(vec![
                            "Led the development ".light_green().bold(),
                            "of a feature enabling users to make charity donations through the app, integrating with external partners to ensure secure, user-friendly transactions.".into()
                        ]),
                        Line::from(vec![
                            "Redesigned a core transaction storage system ".light_green().bold(),
                            "improving performance, reliability, and maintainability while reducing system complexity.".into()
                        ]),
                        Line::from(vec![
                            "Architected and delivered ".light_green().bold(),
                            "a budget tracking service that empowers users to set and monitor overall and category-specific spending limits.".into()
                        ]),
                        Line::from(vec![
                            "Built a real-time monitoring and alerting infrastructure ".light_green().bold(),
                            "using Grafana and Prometheus to detect unusual user behaviours and potential malicious activities.".into()
                        ]),
                        Line::from(vec![
                            "Integrated Singpass".light_green().bold(),
                            ", Singapore’s digital identity service, into the signup flow for Singaporean users, ensuring seamless onboarding experience.".into()
                        ]),
                    ],
                },
                Experience {
                    title: "Senior Software Engineer @ Theta Service Partner".to_owned(),
                    achievements: vec![
                        Line::from(vec![
                            "Led and design and implementation ".light_green().bold(),
                            "of an internal backend framework to streamline workflows and improve engineering efficiency.".into()
                        ]),
                        Line::from(vec![
                            "Optimized loan system performance ".light_green().bold(),
                            "for financial clients achieving a 10x reduction in response time.".into()
                        ]),
                        Line::from(vec![
                            "Mentored and onboarded ".light_green().bold(),
                            "new engineers, helping them ramp up quickly and contribute effectively to client projects.".into()
                        ]),
                    ],
                },
                Experience {
                    title: "Software Engineer @ Theta Service Partner".to_owned(),
                    achievements: vec![
                        Line::from("Created internal tools, optimising team workflows and productivity"),
                        Line::from("Provided support to UK-based client projects and managed server deployment for seamless system operations"),
                    ],
                },
            ],
        };
        frame.render_widget(experiences, exp_area);
        frame.render_widget(
            FunStuffs {
                stuffs: vec![
                    FunStuff {
                        title: "Multiplayer Poker Game".to_owned(),
                        url: "https://www.github.com/yewjung/poker-rust".to_owned(),
                    },
                    FunStuff {
                        title: "Deck API".to_owned(),
                        url: "https://www.github.com/yewjung/deck".to_owned(),
                    },
                ],
            },
            fun_stuff_area,
        );
    }
}

struct Summary;

impl Widget for Summary {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        let paragraph = Paragraph::new("Backend engineer with 5+ years of experience building scalable systems in fintech. Skilled in leading architecture, improving performance, and delivering product features end-to-end.")
            .block(Block::bordered().title("Summary"))
            .fg(Color::White)
            .bg(Color::Black)
            .wrap(Wrap { trim: true });
        paragraph.render(area, buf);
    }
}

struct Experiences<'a, const N: usize> {
    jobs: [Experience<'a>; N],
}

impl<const N: usize> Widget for Experiences<'_, N> {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        let border = Block::bordered()
            .border_type(BorderType::Rounded)
            .title("Experience")
            .title_alignment(Alignment::Left);
        let inner_area = border.inner(area);
        border.render(area, buf);

        let experience_areas: [Rect; N] =
            Layout::vertical(Constraint::from_percentages([50, 25, 25])).areas(inner_area);
        for (exp_area, job) in zip(experience_areas, self.jobs) {
            job.render(exp_area, buf);
        }
    }
}

struct Experience<'a> {
    title: String,
    achievements: Vec<Line<'a>>,
}

impl Widget for Experience<'_> {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        // prepend each achievement with a bullet point
        let achievements: Vec<Line> = self
            .achievements
            .into_iter()
            .map(|ach| {
                Line::from(
                    once("• ".into())
                        .chain(ach.spans.into_iter())
                        .into_iter()
                        .collect::<Vec<_>>(),
                )
            })
            .collect();
        let lines: Vec<_> = once(Line::from(self.title.bold().underlined()))
            .chain(achievements.into_iter())
            .collect();
        let paragraph = Paragraph::new(Text::from(lines))
            .fg(Color::White)
            .bg(Color::Black)
            .wrap(Wrap { trim: true });
        paragraph.render(area, buf);
    }
}

struct FunStuff {
    title: String,
    url: String,
}

impl Widget for FunStuff {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        let [header, content] =
            Layout::vertical([Constraint::Fill(1), Constraint::Fill(1)]).areas(area);
        let lines: Vec<Line> = vec![Line::from(self.title.light_green().bold())];
        Paragraph::new(Text::from(lines)).render(header, buf);
        let link = Hyperlink::new(self.url);
        link.render(content, buf)
    }
}

struct FunStuffs {
    stuffs: Vec<FunStuff>,
}
impl Widget for FunStuffs {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        let border = Block::bordered()
            .border_type(BorderType::Rounded)
            .title("Fun Stuffs".light_magenta())
            .title_alignment(Alignment::Left);
        let inner_area = border.inner(area);
        border.render(area, buf);

        let item_areas: [Rect; 2] =
            Layout::vertical(Constraint::from_percentages([50, 50])).areas(inner_area);
        for (item_area, item) in zip(item_areas, self.stuffs) {
            item.render(item_area, buf);
        }
    }
}
