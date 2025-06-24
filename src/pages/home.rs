use std::cmp::Ordering;

use leptos::prelude::*;
use serde::{Deserialize, Serialize};

use crate::components::*;
use crate::pages::workhistory::WORK_EXPERIENCES;

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct RepoData {
    pub name: String,
    pub html_url: String,
    pub stargazers_count: u32,
    pub language: Option<String>,
}

#[derive(thiserror::Error, Serialize, Deserialize, Debug, Clone)]
pub enum Error {
    #[error("could't fetch repos")]
    RequestError,
    #[error("could't decode repos")]
    DecodeError,
}

impl PartialOrd for RepoData {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for RepoData {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl Ord for RepoData {
    fn cmp(&self, other: &Self) -> Ordering {
        self.stargazers_count.cmp(&other.stargazers_count)
    }
}

impl Eq for RepoData {}

#[component]
pub fn Home() -> impl IntoView {
    view! {
            <Card
                class="grid sm:grid-cols-[auto, auto] grid-cols-[auto]
                sm:grid-rows-[auto, auto] grid-rows-[auto, auto, auto]"
            >
                <div class="sm:col-span-2 mb-8">
                    <Text class="sm:text-lg">
                        <i class="fa-solid fa-globe mr-2"/>
                        "Introduction"
                    </Text>
                </div>
                <div class="grid grid-rows-[auto, auto, auto] gap-4 max-w-lg">
                    <Text class="lg:text-5xl sm:text-4xl text-2xl font-semibold">
                        "Hello, I'm Daimler 👋"
                    </Text>
                    <Text size=TextSize::Lg variant=TextVariant::Dimmed>
                        "I am currently a Junior Data Scientist at "
                        <a
                            href="https://www.priceindustries.com/"
                            target="_blank"
                            class="text-slate-800 dark:text-slate-300"
                        >
                            "Price Industries"
                        </a>
                        ". I am passionate about data, anything AI related and learning new skills!"
                    </Text>
                </div>
                <div
                    class="sm:ml-8 ml-0 sm:mt-0 mt-8 sm:justify-self-end
                    justify-self-center self-center"
                >
                    <img
                        src="/profile.png"
                        alt="Daimelr Garay"
                        class="rounded-full min-[862]:max-w-64 min-[862]:max-h-64
                        md:max-w-56 md:max-h-56  max-w-48 max-h-48 object-cover"
                    />
                </div>
            </Card>
            <div class="grid md:grid-cols-2 grid-cols-1 pt-8 gap-8">
                <div class="flex flex-col gap-6">
                    <Text size=TextSize::Xl weight=TextWeight::Bold>
                        "About Me"
                    </Text>
                    <Text size=TextSize::Lg variant=TextVariant::Dimmed>
                        "I began my journey with programming after highscool, where I pursued and finished an associates degree for Data Science and Machine Learning from Red River College in 2024. I have since continued to learn more about this topic through various certificate programs, given that people and organizations around the world continue to push the boundaries for artificial intelligence. My goal is to grow and learn as much as I can and earn my place in this industry."
                    </Text>
                    <Text size=TextSize::Lg variant=TextVariant::Dimmed>
                        "I am currently learning and teaching myself Rust, coming from Python, the paradigm shift is quite significant but I believe that learning Rust will make me a better programmer overall. "
                    </Text>
                </div>
                    <div class="flex flex-col gap-6">
            <Text size=TextSize::Xl weight=TextWeight::Bold>
                "Work Experience"
            </Text>
            <ul class="flex flex-col gap-4">
                {WORK_EXPERIENCES.iter().map(|exp| view! {
                    <li class="p-4 bg-slate-50 dark:bg-slate-900 rounded-xl shadow">
                        <Text size=TextSize::Lg weight=TextWeight::Bold>
                            {exp.role}
                        </Text>
                        <Text size=TextSize::Md>
                            <span class="text-slate-600 dark:text-slate-400">{exp.company}</span>
                            {" — "}
                            <span class="italic">{exp.period}</span>
                        </Text>
                        <Text size=TextSize::Sm variant=TextVariant::Dimmed>
                            {exp.description}
                        </Text>
                    </li>
                }).collect_view()}
            </ul>
        </div>
    </div>
        }
}
