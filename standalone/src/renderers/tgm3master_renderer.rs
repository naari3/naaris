use graphics::{Context, Text, Transformed};
use piston_window::{G2d, GfxDevice, Glyphs, RenderArgs};
use tetris::TGM3Master;

use super::{Renderer, ToColor, WHITE};

impl Renderer for TGM3Master {
    fn render(
        &mut self,
        args: &RenderArgs,
        c: Context,
        g2d: &mut G2d,
        d: &mut GfxDevice,
        glyphs: &mut Glyphs,
    ) {
        self.inner.render(args, c, g2d, d, glyphs);

        Text::new_color(WHITE.to_color(), 8)
            .draw(
                &format!("{:0>3}", self.get_level()),
                glyphs,
                &c.draw_state,
                c.transform.trans(208.0, 336.0),
                g2d,
            )
            .unwrap();

        let rank = if self.get_level() == 999 {
            999
        } else {
            self.get_level() / 100 * 100 + 100
        };
        Text::new_color(WHITE.to_color(), 8)
            .draw(
                &format!("{: >3}", rank),
                glyphs,
                &c.draw_state,
                c.transform.trans(208.0, 352.0),
                g2d,
            )
            .unwrap();
    }
}
