# `index.test.ts`

**DO NOT MODIFY**. This file has been autogenerated. Run `rome test packages/@romefrontend/js-parser/index.test.ts --update-snapshots` to update.

## `esprima > es2015-arrow-function > migrated_0006`

### `ast`

```javascript
JSRoot {
	comments: Array []
	corrupt: false
	diagnostics: Array []
	directives: Array []
	filename: "input.js"
	hasHoistedVars: false
	interpreter: undefined
	mtime: undefined
	sourceType: "script"
	syntax: Array []
	loc: Object {
		filename: "input.js"
		end: Object {
			column: 0
			index: 19
			line: 2
		}
		start: Object {
			column: 0
			index: 0
			line: 1
		}
	}
	body: Array [
		JSExpressionStatement {
			loc: Object {
				filename: "input.js"
				end: Object {
					column: 18
					index: 18
					line: 1
				}
				start: Object {
					column: 0
					index: 0
					line: 1
				}
			}
			expression: JSArrowFunctionExpression {
				loc: Object {
					filename: "input.js"
					end: Object {
						column: 18
						index: 18
						line: 1
					}
					start: Object {
						column: 0
						index: 0
						line: 1
					}
				}
				head: JSFunctionHead {
					async: false
					hasHoistedVars: false
					rest: undefined
					thisType: undefined
					loc: Object {
						filename: "input.js"
						end: Object {
							column: 4
							index: 4
							line: 1
						}
						start: Object {
							column: 0
							index: 0
							line: 1
						}
					}
					params: Array [
						JSBindingIdentifier {
							name: "e"
							loc: Object {
								filename: "input.js"
								identifierName: "e"
								end: Object {
									column: 1
									index: 1
									line: 1
								}
								start: Object {
									column: 0
									index: 0
									line: 1
								}
							}
						}
					]
				}
				body: JSBlockStatement {
					directives: Array []
					loc: Object {
						filename: "input.js"
						end: Object {
							column: 18
							index: 18
							line: 1
						}
						start: Object {
							column: 5
							index: 5
							line: 1
						}
					}
					body: Array [
						JSLabeledStatement {
							loc: Object {
								filename: "input.js"
								end: Object {
									column: 16
									index: 16
									line: 1
								}
								start: Object {
									column: 7
									index: 7
									line: 1
								}
							}
							label: JSIdentifier {
								name: "label"
								loc: Object {
									filename: "input.js"
									identifierName: "label"
									end: Object {
										column: 12
										index: 12
										line: 1
									}
									start: Object {
										column: 7
										index: 7
										line: 1
									}
								}
							}
							body: JSExpressionStatement {
								loc: Object {
									filename: "input.js"
									end: Object {
										column: 16
										index: 16
										line: 1
									}
									start: Object {
										column: 14
										index: 14
										line: 1
									}
								}
								expression: JSNumericLiteral {
									value: 42
									format: undefined
									loc: Object {
										filename: "input.js"
										end: Object {
											column: 16
											index: 16
											line: 1
										}
										start: Object {
											column: 14
											index: 14
											line: 1
										}
									}
								}
							}
						}
					]
				}
			}
		}
	]
}
```

### `diagnostics`

```
✔ No known problems!

```