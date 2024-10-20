use yazi_macro::render;
use yazi_shared::event::Cmd;

use crate::tasks::Tasks;

struct Opt;

impl From<Cmd> for Opt {
	fn from(_: Cmd) -> Self { Self }
}
impl From<()> for Opt {
	fn from(_: ()) -> Self { Self }
}

impl Tasks {
	#[yazi_codegen::command]
	pub fn toggle(&mut self, _: Opt) {
		self.visible = !self.visible;

		if self.visible {
			self.summaries = self.paginate();
			self.arrow(0);
		}

		render!();
	}
}
