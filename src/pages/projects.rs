use crate::components::{Text, text::TextVariant, text::TextWeight};
use leptos::prelude::*;

#[component]
pub fn Projects() -> impl IntoView {
    view! {
        <div class="flex flex-col items-center justify-center bg-white p-10 rounded-2xl shadow max-w-2xl mx-auto mt-8">
            <Text variant=TextVariant::Black weight=TextWeight::Bold class="text-4xl mb-4">
                "Projects"
            </Text>
            <svg
                width="220"
                height="90"
                viewBox="0 0 220 90"
                fill="none"
                xmlns="http://www.w3.org/2000/svg"
                class="mb-6"
            >
                <rect
                    x="5"
                    y="5"
                    width="210"
                    height="80"
                    rx="16"
                    fill="#f1f5f9"
                    stroke="#cbd5e1"
                    stroke-width="2"
                />
                <text
                    x="50%"
                    y="40"
                    text-anchor="middle"
                    font-size="26"
                    font-family="Inter, sans-serif"
                    fill="#1e293b"
                    font-weight="bold"
                >
                    {"🚧 WIP 🚧"}
                </text>
                <text
                    x="50%"
                    y="70"
                    text-anchor="middle"
                    font-size="16"
                    font-family="Inter, sans-serif"
                    fill="#475569"
                >
                    {"Not done yet!"}
                </text>
            </svg>
        </div>
    }
}
