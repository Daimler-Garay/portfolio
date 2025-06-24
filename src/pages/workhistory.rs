pub struct WorkExperience {
    pub company: &'static str,
    pub role: &'static str,
    pub period: &'static str,
    pub description: &'static str,
}

pub const WORK_EXPERIENCES: &[WorkExperience] = &[
    WorkExperience {
        company: "Price Industries",
        role: "Junior Data Scientist",
        period: "April 2025 – Present",
        description: "Designing and developing custom AI applications for internal business units. Collaborate with cross-functional teams to identify business needs, prototype solutions, and drive innovation in data-driven products.",
    },
    WorkExperience {
        company: "Birchwood Automotive Group",
        role: "Business Analyst",
        period: "July 2024 - March 2025",
        description: "Automated executive reporting processes using Python, improving efficiency. Applied statistical analysis to optimize product pricing strategies, ensuring sustained market competitiveness, profitability and customer satisfaction.",
    },
    WorkExperience {
        company: "Deco Windshield Repair",
        role: "Glass Technician",
        period: "April 2023 - September 2023",
        description: "Delivered professional windshield repair and replacement services while providing excellent customer support. Proactively identified customer needs, recommended appropriate solutions, and successfully promoted company products and services to increase sales.",
    },
];
