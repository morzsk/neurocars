use crate::{
    racer::{
        RACER_ACTION_OUTPUT_COUNT, RACER_SENSOR_INPUT_COUNT, RACER_SENSOR_MAX_DISTANCE,
        RacerAction, RacerSensor, RacerSensorHit,
    },
    utils::AxisInput,
};
use macroquad::rand::gen_range;

pub type Input = [f32; RACER_SENSOR_INPUT_COUNT];
pub type Output = [f32; RACER_ACTION_OUTPUT_COUNT];

#[derive(Clone, Debug)]
pub struct Genome {
    pub weights: [[f32; RACER_SENSOR_INPUT_COUNT]; RACER_ACTION_OUTPUT_COUNT],
    pub biases: [f32; RACER_ACTION_OUTPUT_COUNT],
}

pub fn random_genome() -> Genome {
    Genome {
        weights: std::array::from_fn(|_| std::array::from_fn(|_| gen_range(-1.0, 1.0))),
        biases: std::array::from_fn(|_| gen_range(-1.0, 1.0)),
    }
}

pub fn encode_sensors_to_input(sensors: &RacerSensor) -> Input {
    [
        encode_sensor(sensors.left.as_ref()),
        encode_sensor(sensors.middle.as_ref()),
        encode_sensor(sensors.right.as_ref()),
    ]
}

fn encode_sensor(hit: Option<&RacerSensorHit>) -> f32 {
    hit.map_or(0.0, |hit| normalize_sensor_distance(hit.distance))
}

fn normalize_sensor_distance(distance: f32) -> f32 {
    1.0 - (distance / RACER_SENSOR_MAX_DISTANCE).clamp(0.0, 1.0)
}

pub fn evaluate(genome: &Genome, input: Input) -> Output {
    let mut output = [0.0; RACER_ACTION_OUTPUT_COUNT];

    for ((output_value, weights), bias) in
        output.iter_mut().zip(&genome.weights).zip(&genome.biases)
    {
        let value = input
            .iter()
            .zip(weights)
            .fold(*bias, |sum, (input, weight)| sum + input * weight);

        *output_value = value.tanh();
    }

    output
}

pub fn decode_output_to_action(output: Output) -> RacerAction {
    let [throttle, steer] = output;

    RacerAction {
        throttle: AxisInput::new(throttle).expect("throttle output must be between -1 and 1"),
        steer: AxisInput::new(steer).expect("steer output must be between -1 and 1"),
    }
}
