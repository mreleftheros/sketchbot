use sketchbot::{SketchBot, sleep::sleep_s};

fn main() {
  sleep_s(3);

  let bot: SketchBot = SketchBot::new();

  // bot.sketch_squares();
  // sleep_s(5);
  
  bot.sketch_from_file("sketches/recorded.json");
}

