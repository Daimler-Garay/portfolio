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
    #[error("couldn't fetch repos")]
    RequestError,
    #[error("couldn't decode repos")]
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
    let technologies = vec![
        "Rust",
        "Python",
        "SQL",
        "Pandas",
        "TensorFlow",
        "Excel",
        "Power BI",
        "Git",
    ];

    view! {
        // Card: Name, title, photo, tech stack
        <Card variant=CardVariant::Outlined class="flex flex-col md:flex-row items-center bg-white dark:bg-white rounded-2xl shadow-xl p-8 max-w-3xl mx-auto">
            // PROFILE PHOTO
            <div class="flex-shrink-0 mb-6 md:mb-0 md:mr-8">
                <img
                    src="/profile.png"
                    alt="Daimler Garay"
                    class="rounded-full w-40 h-40 object-cover border-4 border-indigo-100 shadow"
                />
            </div>
            // TEXT SECTION
            <div class="flex flex-col items-center md:items-start">
                <Text variant=TextVariant::Black class="text-4xl font-bold mb-2">
                    "Daimler Garay"
                </Text>
                <Text variant=TextVariant::Black class="text-lg font-semibold mb-3">
                    "Junior Data Scientist"
                </Text>
                <div class="flex flex-wrap gap-2 mb-4 justify-center md:justify-start">
                    {technologies.into_iter().map(|tech| view! {
                        <span class="bg-indigo-50 text-indigo-800 text-xs font-semibold px-3 py-1 rounded-full shadow-sm">
                            {tech}
                        </span>
                    }).collect_view()}
                </div>
                <Text variant=TextVariant::Black class="text-center md:text-left mb-2">
                    "I build data-driven solutions and love learning new technologies. Passionate about AI, analytics, and code that makes a difference."
                </Text>
                <div class="flex gap-4 mt-3">
                    <a href="https://github.com/Daimler-Garay" target="_blank">
                        <i class="fa-brands fa-github text-2xl text-black hover:text-indigo-700"></i>
                    </a>
                    <a href="https://www.linkedin.com/in/daimler-chrysler-406341243/" target="_blank">
                        <i class="fa-brands fa-linkedin text-2xl text-black hover:text-indigo-700"></i>
                    </a>
                </div>
            </div>
        </Card>

        // About and Work Experience section
        <div class="grid md:grid-cols-2 grid-cols-1 pt-12 gap-8 max-w-4xl mx-auto">
            <div class="flex flex-col gap-6 items-center justify-center">
                <iframe
                    src="https://www.google.com/maps/d/u/0/embed?mid=1SVhoFmeDvibC2EzX8X16KUaZXY_h-yM&ehbc=2E312F"
                    width="340"
                    height="580"
                    style="border:0;"
                    loading="lazy"
                    referrerpolicy="no-referrer-when-downgrade"
                    class="rounded-lg shadow border w-full"
                ></iframe>
                <Text variant=TextVariant::Black class="text-sm mt-2 text-center">
                    "A map of the countries and cities I've visited."
                </Text>
            </div>
            <div class="flex flex-col gap-6">
                <Text variant=TextVariant::Black class="text-2xl font-bold">
                    "Work Experience"
                </Text>
                <ul class="flex flex-col gap-4">
                    {WORK_EXPERIENCES.iter().map(|exp| view! {
                        <li class="p-4 bg-white dark:bg-white rounded-xl shadow">
                            <Text variant=TextVariant::Black class="text-lg font-bold">
                                {exp.role}
                            </Text>
                            <Text variant=TextVariant::Black class="text-md">
                                <span>{exp.company}</span>
                                {" — "}
                                <span class="italic">{exp.period}</span>
                            </Text>
                            <Text variant=TextVariant::Black class="text-sm">
                                {exp.description}
                            </Text>
                        </li>
                    }).collect_view()}
                </ul>
            </div>
        </div>
    }
}
