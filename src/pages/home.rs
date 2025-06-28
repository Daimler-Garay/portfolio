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
        "Python",
        "Rust",
        "SQL",
        "Pandas",
        "TensorFlow",
        "Excel",
        "Power BI",
    ];

    view! {
        // --- PROFILE CARD ---
        <Card
            variant=CardVariant::Outlined
            class="flex flex-col md:flex-row items-center bg-white dark:bg-white rounded-2xl shadow-xl p-8 w-full max-w-10xl mx-auto"
        >
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
                    {technologies
                        .into_iter()
                        .map(|tech| {
                            view! {
                                <span class="bg-indigo-50 text-indigo-800 text-xs font-semibold px-3 py-1 rounded-full shadow-sm">
                                    {tech}
                                </span>
                            }
                        })
                        .collect_view()}
                </div>
                <Text variant=TextVariant::Black class="text-center md:text-left mb-2">
                    "I build data-driven solutions and love learning new technologies. Passionate about AI, analytics, and code that makes a difference."
                </Text>
                <div class="flex gap-4 mt-3">
                    <a href="https://github.com/Daimler-Garay" target="_blank">
                        <i class="fa-brands fa-github text-2xl text-black hover:text-indigo-700"></i>
                    </a>
                    <a
                        href="https://www.linkedin.com/in/daimler-chrysler-406341243/"
                        target="_blank"
                    >
                        <i class="fa-brands fa-linkedin text-2xl text-black hover:text-indigo-700"></i>
                    </a>
                </div>
            </div>
        </Card>

        // --- GRID: HIGHLIGHT PROJECT & WORK EXPERIENCE ---
        <div class="grid md:grid-cols-2 grid-cols-1 pt-12 gap-8 max-w-10xl mx-auto w-full">
            // --- LEFT: Highlight Project Section ---
            <div class="flex flex-col w-full">
                <Text variant=TextVariant::Black class="text-2xl font-bold mb-3 text-center">
                    "Project Spotlight"
                </Text>
                <div class="relative flex flex-col gap-4 items-center justify-center
                bg-white dark:bg-white
                rounded-2xl shadow-2xl  border-black-100 
                p-6 md:p-8 
                min-h-[580px] w-full
                
                transition-transform duration-200 hover:scale-[1.025]">
                    <div class="w-full flex justify-center">
                        <img
                            src="/brain_tumor.jpg"
                            alt="Brain tumor"
                            class="w-full max-w-xl rounded-xl shadow-xl border-4 border-indigo-100 mb-2 object-cover transition-transform duration-200 hover:scale-105"
                            style="width: 250px; height: 180px;"
                        />
                    </div>
                    <Text
                        variant=TextVariant::Black
                        class="text-lg font-bold text-indigo-900 mt-2 text-center"
                    >
                        "Brain Tumor Classification"
                    </Text>
                    <Text variant=TextVariant::Black class="text-md text-center text-slate-700">
                        "This project developed a CNN to classify brain MRI images into four categories (glioma, meningioma, pituitary, or no tumor), standardizing and augmenting data from multiple sources. The model achieved high accuracy (up to 99%) and low loss, successfully predicting all test cases. Further tuning could enhance performance, but current results are strong."
                    </Text>
                    <a
                        href="https://github.com/Daimler-Garay/Brain_Tumor"
                        class="inline-flex items-center gap-2 mt-2 px-5 py-2 bg-indigo-700 text-white font-semibold rounded-xl shadow-lg hover:bg-indigo-900 hover:shadow-xl transition-all duration-200 text-md"
                    >
                        <i class="fa-solid fa-arrow-up-right-from-square text-white"></i>
                        "View Project"
                    </a>
                </div>
            </div>
            // --- RIGHT: Work Experience Section ---
            <div class="flex flex-col w-full">
                <Text variant=TextVariant::Black class="text-2xl font-bold mb-3 text-center">
                    "Work Experience"
                </Text>
                <div class="relative flex flex-col gap-4 items-center justify-center
                bg-white dark:bg-white
                rounded-2xl shadow-2xl  border-black-100 
                p-6 md:p-8
                min-h-[580px] w-full
                box-border">
                    <ul class="flex flex-col gap-4 h-full w-full overflow-y-auto pr-2 max-h-[500px]">
                        {WORK_EXPERIENCES
                            .iter()
                            .map(|exp| {
                                view! {
                                    <li class="p-4 bg-white dark:bg-white rounded-xl shadow border-1">
                                        <Text variant=TextVariant::Black class="text-lg font-bold">
                                            {exp.role}
                                        </Text>
                                        <Text
                                            size=TextSize::Sm
                                            variant=TextVariant::Black
                                            class="text-md italic"
                                        >
                                            {exp.company}
                                        </Text>
                                        <Text
                                            size=TextSize::Sm
                                            variant=TextVariant::Black
                                            class="text-md"
                                        >
                                            {exp.period}
                                        </Text>
                                        <Text variant=TextVariant::Black class="text-sm">
                                            {exp.description}
                                        </Text>
                                    </li>
                                }
                            })
                            .collect_view()}
                    </ul>
                </div>
            </div>
        </div>

        // --- MAP SECTION ---
        <div class="flex flex-col w-full max-w-10xl items-center justify-center pt-12">
            <iframe
                width="1090"
                height="376"
                src="https://maphub.net/embed_h/QkiorZoOGIddnmib?panel=0&panel_closed=1&button=0"
                frameborder="1"
                class="rounded-lg shadow border"
            ></iframe>
            <Text variant=TextVariant::Black class="text-sm mt-2 text-center">
                "A map of the countries and cities I've visited."
            </Text>
        </div>
    }
}
