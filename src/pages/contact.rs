use leptos::prelude::*;

use crate::components::{Text, TextSize, text::TextVariant, text::TextWeight};

#[component]
pub fn Contact() -> impl IntoView {
    view! {
        <div class="flex flex-col items-center justify-center bg-white p-10 rounded-2xl shadow max-w-lg mx-auto mt-8">
            <Text variant=TextVariant::Black weight=TextWeight::SemiBold class="text-4xl mb-4">
                "Contact Me"
            </Text>
            <Text variant=TextVariant::Black size=TextSize::Xl class="mb-4 text-center">
                "Feel free to reach out to me via email!"
            </Text>
            <a
                href="mailto:garay.daimlerchryslerfernandez@gmail.com"
                class="px-6 py-2 rounded-lg bg-black text-white font-semibold text-lg shadow hover:bg-indigo-700 transition-colors duration-150"
            >
                "Send Email"
            </a>
        </div>
    }
}
