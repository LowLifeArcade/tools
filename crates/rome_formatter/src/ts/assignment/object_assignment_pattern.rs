use crate::{
	empty_element, format_elements, group_elements, join_elements, soft_indent,
	soft_line_break_or_space, space_token, FormatElement, FormatResult, Formatter, ToFormatElement,
};
use rslint_parser::ast::{
	JsAnyObjectAssignmentPatternMember, JsObjectAssignmentPattern,
	JsObjectAssignmentPatternProperty, JsObjectAssignmentPatternRest,
	JsObjectAssignmentPatternShorthandProperty,
};

impl ToFormatElement for JsObjectAssignmentPattern {
	fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
		let properties = formatter.format_separated(self.properties())?;
		Ok(group_elements(format_elements![
			formatter.format_token(&self.l_curly_token()?)?,
			space_token(),
			soft_indent(join_elements(soft_line_break_or_space(), properties)),
			space_token(),
			formatter.format_token(&self.r_curly_token()?)?,
		]))
	}
}

impl ToFormatElement for JsAnyObjectAssignmentPatternMember {
	fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
		match self {
			JsAnyObjectAssignmentPatternMember::JsObjectAssignmentPatternShorthandProperty(
				object_assignment_pattern_shorthand_property,
			) => object_assignment_pattern_shorthand_property.to_format_element(formatter),
			JsAnyObjectAssignmentPatternMember::JsObjectAssignmentPatternProperty(
				object_assignment_pattern_property,
			) => object_assignment_pattern_property.to_format_element(formatter),
			JsAnyObjectAssignmentPatternMember::JsObjectAssignmentPatternRest(
				object_assignment_pattern_rest,
			) => object_assignment_pattern_rest.to_format_element(formatter),
			JsAnyObjectAssignmentPatternMember::JsUnknownAssignment(_) => todo!(),
		}
	}
}

impl ToFormatElement for JsObjectAssignmentPatternShorthandProperty {
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

impl ToFormatElement for JsObjectAssignmentPatternProperty {
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

impl ToFormatElement for JsObjectAssignmentPatternRest {
	fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
		Ok(format_elements![
			formatter.format_token(&self.dotdotdot_token()?)?,
			formatter.format_node(self.target()?)?,
		])
	}
}
