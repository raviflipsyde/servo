/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

<%namespace name="helpers" file="/helpers.mako.rs" />

<% from data import Method %>

<% data.new_style_struct("Border", inherited=False, gecko_ffi_name="nsStyleBorder",
                   additional_methods=[Method("border_" + side + "_has_nonzero_width",
                                              "bool") for side in ["top", "right", "bottom", "left"]]) %>

% for side in ["top", "right", "bottom", "left"]:
    ${helpers.predefined_type("border-%s-color" % side, "CSSColor", "::cssparser::Color::CurrentColor")}
% endfor

% for side in ["top", "right", "bottom", "left"]:
    ${helpers.predefined_type("border-%s-style" % side, "BorderStyle", "specified::BorderStyle::none", need_clone=True)}
% endfor

% for side in ["top", "right", "bottom", "left"]:
    <%helpers:longhand name="border-${side}-width">
        use app_units::Au;
        use cssparser::ToCss;
        use std::fmt;

        impl ToCss for SpecifiedValue {
            fn to_css<W>(&self, dest: &mut W) -> fmt::Result where W: fmt::Write {
                self.0.to_css(dest)
            }
        }

        #[inline]
        pub fn parse(_context: &ParserContext, input: &mut Parser)
                               -> Result<SpecifiedValue, ()> {
            specified::parse_border_width(input).map(SpecifiedValue)
        }
        #[derive(Debug, Clone, PartialEq, HeapSizeOf)]
        pub struct SpecifiedValue(pub specified::Length);
        pub mod computed_value {
            use app_units::Au;
            pub type T = Au;
        }
        #[inline] pub fn get_initial_value() -> computed_value::T {
            Au::from_px(3)  // medium
        }

        impl ToComputedValue for SpecifiedValue {
            type ComputedValue = computed_value::T;

            #[inline]
            fn to_computed_value<Cx: TContext>(&self, context: &Cx) -> computed_value::T {
                self.0.to_computed_value(context)
            }
        }
    </%helpers:longhand>
% endfor

// FIXME(#4126): when gfx supports painting it, make this Size2D<LengthOrPercentage>
% for corner in ["top-left", "top-right", "bottom-right", "bottom-left"]:
    ${helpers.predefined_type("border-" + corner + "-radius", "BorderRadiusSize",
                              "computed::BorderRadiusSize::zero()",
                              "parse")}
% endfor
