use chrono::TimeDelta;
use uuid::{NoContext, Timestamp, Uuid};

#[derive(strum_macros::Display, PartialEq)]
pub enum GardeningTaskPriority {
    Low,
    Medium,
    High,
}

impl GardeningTaskPriority {
    pub fn get_color(value: GardeningTaskPriority) -> &'static str {
        if value == GardeningTaskPriority::High {
            return "bg-red-100 text-red-800 border-red-200";
        } else if value == GardeningTaskPriority::Medium {
            return "bg-yellow-100 text-yellow-800 border-yellow-200";
        } else if value == GardeningTaskPriority::Low {
            return "bg-green-100 text-green-800 border-green-200";
        } else {
            return "bg-gray-100 text-gray-800 border-gray-200";
        }
    }
}

pub struct GardeningTask {
    pub id: Uuid,
    pub title: String,
    pub priority: GardeningTaskPriority,
    pub time_required: TimeDelta,
    pub description: String,
    pub detailed_steps: Vec<String>,
    pub tips: String,
}

impl GardeningTask {
    pub fn new(
        title: String,
        priority: GardeningTaskPriority,
        time_required: TimeDelta,
        description: String,
        detailed_steps: Vec<String>,
        tips: String,
    ) -> GardeningTask {
        let id = Uuid::new_v7(Timestamp::now(NoContext));
        GardeningTask {
            id,
            title,
            priority,
            time_required,
            description,
            detailed_steps,
            tips,
        }
    }
}
