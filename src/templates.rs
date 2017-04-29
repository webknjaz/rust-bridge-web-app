use std::collections::BTreeMap;

use motivations::MOTIVATIONS;

use pencil::{Request, PencilResult};

use pick_one::pick_one_str;


pub fn motivation(request: &mut Request) -> PencilResult {
    let mut context = BTreeMap::new();
    let motivation = pick_one_str(&MOTIVATIONS).to_string();
    context.insert("motivation".to_string(), motivation);
    request.app.render_template("motivation.html", &context)
}
