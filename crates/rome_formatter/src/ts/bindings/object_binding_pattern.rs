use crate::{
	empty_element, format_elements, group_elements, if_group_breaks, join_elements, soft_indent,
	soft_line_break_or_space, space_token, token, FormatElement, FormatResult, Formatter,
	ToFormatElement,
};
use rslint_parser::ast::{
	JsAnyObjectBindingPatternMember, JsObjectBindingPattern, JsObjectBindingPatternProperty,
	JsObjectBindingPatternRest, JsObjectBindingPatternShorthandProperty,
};

impl ToFormatElement for JsObjectBindingPattern {
	fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
		let properties = formatter.format_separated(self.properties())?;

		Ok(group_elements(format_elements![
			formatter.format_token(&self.l_curly_token()?)?,
			space_token(),
			soft_indent(format_elements![
				join_elements(soft_line_break_or_space(), properties),
				if_group_breaks(token(",")),
			]),
			space_token(),
			formatter.format_token(&self.r_curly_token()?)?
		]))
	}
}

impl ToFormatElement for JsAnyObjectBindingPatternMember {
	fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
		match self {
			JsAnyObjectBindingPatternMember::JsObjectBindingPatternProperty(
				object_binding_pattern_property,
			) => object_binding_pattern_property.to_format_element(formatter),
			JsAnyObjectBindingPatternMember::JsObjectBindingPatternRest(
				object_binding_pattern_rest,
			) => object_binding_pattern_rest.to_format_element(formatter),
			JsAnyObjectBindingPatternMember::JsObjectBindingPatternShorthandProperty(
				object_binding_pattern_shorthand_property,
			) => object_binding_pattern_shorthand_property.to_format_element(formatter),
			JsAnyObjectBindingPatternMember::JsIdentifierBinding(identifier_binding) => {
				identifier_binding.to_format_element(formatter)
			}
			JsAnyObjectBindingPatternMember::JsUnknownBinding(_) => todo!(),
		}
	}
}

impl ToFormatElement for JsObjectBindingPatternProperty {
	fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
		let init_node = if let Some(node) = self.init() {
			format_elements![space_token(), formatter.format_node(node)?]
		} else {
			empty_element()
		};
		Ok(format_elements![
			formatter.format_node(self.member()?)?,
			formatter.format_token(&self.colon_token()?)?,
			space_token(),
			formatter.format_node(self.pattern()?)?,
			init_node,
		])
	}
}

impl ToFormatElement for JsObjectBindingPatternRest {
	fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
		Ok(format_elements![
			formatter.format_token(&self.dotdotdot_token()?)?,
			formatter.format_node(self.binding()?)?,
		])
	}
}

impl ToFormatElement for JsObjectBindingPatternShorthandProperty {
	fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
		let init_node = if let Some(node) = self.init() {
			format_elements![space_token(), formatter.format_node(node)?]
		} else {
			empty_element()
		};
		Ok(format_elements![
			formatter.format_node(self.identifier()?)?,
			init_node
		])
	}
}
