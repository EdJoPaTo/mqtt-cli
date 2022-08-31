use chrono::{DateTime, Local};
use json::JsonValue;

use crate::json_view;
use crate::mqtt::{HistoryEntry, Payload};

#[allow(clippy::cast_precision_loss)]
fn parse_time_to_chart_x(time: &DateTime<Local>) -> f64 {
    time.timestamp_millis() as f64
}
struct Point {
    time: DateTime<Local>,
    y: f64,
}

impl Point {
    fn parse(entry: &HistoryEntry, json_selector: &[usize]) -> Option<Self> {
        let time = entry.time.as_optional()?;
        let y = match &entry.payload {
            Payload::NotUtf8(_) => None,
            Payload::String(str) => str.parse::<f64>().ok(),
            Payload::Json(json) => {
                let json = json_view::get_selected_subvalue(json, json_selector).unwrap_or(json);
                match json {
                    JsonValue::Number(num) => Some((*num).into()),
                    JsonValue::Boolean(true) => Some(1.0),
                    JsonValue::Boolean(false) => Some(0.0),
                    #[allow(clippy::cast_precision_loss)]
                    JsonValue::Array(arr) => Some(arr.len() as f64),
                    JsonValue::Short(str) => str.parse::<f64>().ok(),
                    JsonValue::String(str) => str.parse::<f64>().ok(),
                    JsonValue::Null | JsonValue::Object(_) => None,
                }
            }
        }?;
        if y.is_finite() {
            Some(Self { time, y })
        } else {
            None
        }
    }

    fn as_graph_point(&self) -> (f64, f64) {
        let x = parse_time_to_chart_x(&self.time);
        (x, self.y)
    }
}

/// Dataset of Points showable by the graph. Ensures to create a useful graph (has at least 2 points)
pub struct GraphData {
    pub data: Vec<(f64, f64)>,
    pub first_time: DateTime<Local>,
    pub last_time: DateTime<Local>,
    pub x_max: f64,
    pub x_min: f64,
    pub y_max: f64,
    pub y_min: f64,
}

impl GraphData {
    pub fn parse(entries: &[HistoryEntry], json_selector: &[usize]) -> Option<Self> {
        let points = entries
            .iter()
            .filter_map(|o| Point::parse(o, json_selector))
            .collect::<Vec<_>>();

        if points.len() < 2 {
            return None;
        }

        let first_time = points.first().unwrap().time;
        let last_time = points.last().unwrap().time;
        let x_min = parse_time_to_chart_x(&first_time);
        let x_max = parse_time_to_chart_x(&last_time);

        let mut data = Vec::with_capacity(points.len());
        let mut y_min = points.first().unwrap().y;
        let mut y_max = y_min;
        for point in points {
            y_min = y_min.min(point.y);
            y_max = y_max.max(point.y);
            data.push(point.as_graph_point());
        }

        Some(Self {
            data,
            first_time,
            last_time,
            x_max,
            x_min,
            y_max,
            y_min,
        })
    }
}
