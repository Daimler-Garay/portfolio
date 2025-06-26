use crate::components::{Text, text::TextVariant, text::TextWeight};
use leptos::prelude::*;

#[component]
pub fn Projects() -> impl IntoView {
    view! {
        <div class="flex flex-col items-center justify-center bg-white p-10 rounded-2xl shadow max-w-2xl mx-auto mt-8">
            <Text variant=TextVariant::Black weight=TextWeight::Bold class="text-4xl mb-4">
                "Projects"
            </Text>
            <Text variant=TextVariant::Black class="text-lg text-center">
                "Here you can showcase your projects. Add project cards or links here!"
            </Text>
        </div>
    }
}
