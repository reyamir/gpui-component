use gpui::{
    div, ClickEvent, IntoElement, ParentElement, Render, RenderOnce, Styled, ViewContext,
    WindowContext,
};

use ui::{checkbox::Checkbox, h_flex, v_flex, Disableable as _, Selection};

use super::story_case;

pub struct CheckboxStory {
    check1: Selection,
    check2: Selection,
    check3: Selection,
}

impl CheckboxStory {
    pub(crate) fn new(_cx: &mut WindowContext) -> Self {
        Self {
            check1: Selection::Unselected,
            check2: Selection::Indeterminate,
            check3: Selection::Selected,
        }
    }

    #[allow(unused)]
    fn on_click(sel: &Selection, cx: &mut WindowContext) {
        println!("Check value changed: {}", sel);
    }
}

impl Render for CheckboxStory {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        story_case(
            "Checkbox",
            "A control that allows the user to toggle between checked and not checked.",
        )
        .child(
            v_flex().items_start().justify_start().gap_6().child(
                h_flex()
                    .items_center()
                    .gap_6()
                    .child(
                        Checkbox::new("check1", cx)
                            .checked(self.check1)
                            .on_click(cx.listener(|v, _, _| {
                                v.check1 = v.check1.inverse();
                            })),
                    )
                    .child(
                        Checkbox::new("check2", cx)
                            .checked(self.check2)
                            .label("Subscribe to newsletter")
                            .on_click(cx.listener(|v, _, _| {
                                v.check2 = v.check2.inverse();
                            })),
                    )
                    .child(
                        Checkbox::new("check3", cx)
                            .checked(self.check3)
                            .label("Remember me")
                            .on_click(cx.listener(|v, _, _| {
                                v.check3 = v.check3.inverse();
                            })),
                    ),
            ),
        )
        .child(
            h_flex().items_center().gap_4().child(
                h_flex()
                    .items_center()
                    .gap_6()
                    .child(
                        Checkbox::new("check3", cx)
                            .label("Disabled Checked")
                            .checked(Selection::Selected)
                            .disabled(true),
                    )
                    .child(
                        Checkbox::new("check3_1", cx)
                            .label("Disabled Unchecked")
                            .checked(Selection::Unselected)
                            .disabled(true),
                    )
                    .child(
                        Checkbox::new("check3_2", cx)
                            .label("Disabled Indeterminate")
                            .checked(Selection::Indeterminate)
                            .disabled(true),
                    ),
            ),
        )
    }
}