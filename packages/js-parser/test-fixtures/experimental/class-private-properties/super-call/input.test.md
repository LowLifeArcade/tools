# `index.test.ts`

**DO NOT MODIFY**. This file has been autogenerated. Run `rome test packages/js-parser/index.test.ts --update-snapshots` to update.

## `experimental > class-private-properties > super-call`

### `ast`

```javascript
JSRoot {
	comments: Array []
	corrupt: false
	directives: Array []
	filename: "experimental/class-private-properties/super-call/input.js"
	hasHoistedVars: false
	interpreter: undefined
	mtime: undefined
	sourceType: "script"
	syntax: Array []
	loc: Object {
		filename: "experimental/class-private-properties/super-call/input.js"
		end: Object {
			column: 1
			index: 95
			line: 7
		}
		start: Object {
			column: 0
			index: 0
			line: 1
		}
	}
	diagnostics: Array [
		Object {
			origins: Array [Object {category: "parse/js"}]
			description: Object {
				category: "parse/js"
				message: MARKUP {parts: Array [RAW_MARKUP {value: "super() is only valid inside a class constructor of a subclass"}]}
				advice: Array [
					log {
						category: "info"
						text: MARKUP {parts: Array [RAW_MARKUP {value: "Maybe a typo in the method name ('constructor') or not extending another class?"}]}
					}
				]
			}
			location: Object {
				filename: "experimental/class-private-properties/super-call/input.js"
				mtime: undefined
				sourceText: undefined
				end: Object {
					column: 18
					index: 80
					line: 4
				}
				start: Object {
					column: 13
					index: 75
					line: 4
				}
			}
		}
	]
	body: Array [
		JSClassDeclaration {
			id: JSBindingIdentifier {
				name: "A"
				loc: Object {
					filename: "experimental/class-private-properties/super-call/input.js"
					identifierName: "A"
					end: Object {
						column: 7
						index: 7
						line: 1
					}
					start: Object {
						column: 6
						index: 6
						line: 1
					}
				}
			}
			loc: Object {
				filename: "experimental/class-private-properties/super-call/input.js"
				end: Object {
					column: 1
					index: 95
					line: 7
				}
				start: Object {
					column: 0
					index: 0
					line: 1
				}
			}
			meta: JSClassHead {
				implements: undefined
				superTypeParameters: undefined
				typeParameters: undefined
				loc: Object {
					filename: "experimental/class-private-properties/super-call/input.js"
					end: Object {
						column: 1
						index: 95
						line: 7
					}
					start: Object {
						column: 0
						index: 0
						line: 1
					}
				}
				superClass: JSReferenceIdentifier {
					name: "B"
					loc: Object {
						filename: "experimental/class-private-properties/super-call/input.js"
						identifierName: "B"
						end: Object {
							column: 17
							index: 17
							line: 1
						}
						start: Object {
							column: 16
							index: 16
							line: 1
						}
					}
				}
				body: Array [
					JSClassMethod {
						kind: "constructor"
						key: JSStaticPropertyKey {
							value: JSIdentifier {
								name: "constructor"
								loc: Object {
									filename: "experimental/class-private-properties/super-call/input.js"
									identifierName: "constructor"
									end: Object {
										column: 13
										index: 33
										line: 2
									}
									start: Object {
										column: 2
										index: 22
										line: 2
									}
								}
							}
							loc: Object {
								filename: "experimental/class-private-properties/super-call/input.js"
								end: Object {
									column: 13
									index: 33
									line: 2
								}
								start: Object {
									column: 2
									index: 22
									line: 2
								}
							}
						}
						loc: Object {
							filename: "experimental/class-private-properties/super-call/input.js"
							end: Object {
								column: 3
								index: 93
								line: 6
							}
							start: Object {
								column: 2
								index: 22
								line: 2
							}
						}
						head: JSFunctionHead {
							async: false
							generator: false
							hasHoistedVars: false
							params: Array []
							rest: undefined
							returnType: undefined
							thisType: undefined
							typeParameters: undefined
							loc: Object {
								filename: "experimental/class-private-properties/super-call/input.js"
								end: Object {
									column: 15
									index: 35
									line: 2
								}
								start: Object {
									column: 13
									index: 33
									line: 2
								}
							}
						}
						meta: JSClassPropertyMeta {
							abstract: false
							accessibility: undefined
							optional: false
							readonly: false
							static: false
							typeAnnotation: undefined
							start: Object {
								column: 2
								index: 22
								line: 2
							}
							loc: Object {
								filename: "experimental/class-private-properties/super-call/input.js"
								end: Object {
									column: 13
									index: 33
									line: 2
								}
								start: Object {
									column: 2
									index: 22
									line: 2
								}
							}
						}
						body: JSBlockStatement {
							directives: Array []
							loc: Object {
								filename: "experimental/class-private-properties/super-call/input.js"
								end: Object {
									column: 3
									index: 93
									line: 6
								}
								start: Object {
									column: 16
									index: 36
									line: 2
								}
							}
							body: Array [
								JSClassDeclaration {
									id: JSBindingIdentifier {
										name: "C"
										loc: Object {
											filename: "experimental/class-private-properties/super-call/input.js"
											identifierName: "C"
											end: Object {
												column: 11
												index: 49
												line: 3
											}
											start: Object {
												column: 10
												index: 48
												line: 3
											}
										}
									}
									loc: Object {
										filename: "experimental/class-private-properties/super-call/input.js"
										end: Object {
											column: 5
											index: 89
											line: 5
										}
										start: Object {
											column: 4
											index: 42
											line: 3
										}
									}
									meta: JSClassHead {
										implements: undefined
										superTypeParameters: undefined
										typeParameters: undefined
										loc: Object {
											filename: "experimental/class-private-properties/super-call/input.js"
											end: Object {
												column: 5
												index: 89
												line: 5
											}
											start: Object {
												column: 4
												index: 42
												line: 3
											}
										}
										superClass: JSReferenceIdentifier {
											name: "D"
											loc: Object {
												filename: "experimental/class-private-properties/super-call/input.js"
												identifierName: "D"
												end: Object {
													column: 21
													index: 59
													line: 3
												}
												start: Object {
													column: 20
													index: 58
													line: 3
												}
											}
										}
										body: Array [
											JSClassPrivateProperty {
												key: JSPrivateName {
													id: JSIdentifier {
														name: "foo"
														loc: Object {
															filename: "experimental/class-private-properties/super-call/input.js"
															identifierName: "foo"
															end: Object {
																column: 10
																index: 72
																line: 4
															}
															start: Object {
																column: 7
																index: 69
																line: 4
															}
														}
													}
													loc: Object {
														filename: "experimental/class-private-properties/super-call/input.js"
														end: Object {
															column: 10
															index: 72
															line: 4
														}
														start: Object {
															column: 6
															index: 68
															line: 4
														}
													}
												}
												value: JSCallExpression {
													arguments: Array []
													loc: Object {
														filename: "experimental/class-private-properties/super-call/input.js"
														end: Object {
															column: 20
															index: 82
															line: 4
														}
														start: Object {
															column: 13
															index: 75
															line: 4
														}
													}
													callee: JSSuper {
														loc: Object {
															filename: "experimental/class-private-properties/super-call/input.js"
															end: Object {
																column: 18
																index: 80
																line: 4
															}
															start: Object {
																column: 13
																index: 75
																line: 4
															}
														}
													}
												}
												typeAnnotation: undefined
												loc: Object {
													filename: "experimental/class-private-properties/super-call/input.js"
													end: Object {
														column: 21
														index: 83
														line: 4
													}
													start: Object {
														column: 6
														index: 68
														line: 4
													}
												}
												meta: JSClassPropertyMeta {
													abstract: false
													accessibility: undefined
													optional: false
													readonly: false
													static: false
													typeAnnotation: undefined
													start: Object {
														column: 6
														index: 68
														line: 4
													}
													loc: Object {
														filename: "experimental/class-private-properties/super-call/input.js"
														end: Object {
															column: 10
															index: 72
															line: 4
														}
														start: Object {
															column: 6
															index: 68
															line: 4
														}
													}
												}
											}
										]
									}
								}
							]
						}
					}
				]
			}
		}
	]
}
```

### `diagnostics`

```

 experimental/class-private-properties/super-call/input.js:4:13 parse/js ━━━━━━━━━━━━━━━━━━━━━━━━━━━

  ✖ super() is only valid inside a class constructor of a subclass

    2 │   constructor() {
    3 │     class C extends D {
  > 4 │       #foo = super();
      │              ^^^^^
    5 │     }
    6 │   }

  ℹ Maybe a typo in the method name ('constructor') or not extending another class?

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

✖ Found 1 problem

```