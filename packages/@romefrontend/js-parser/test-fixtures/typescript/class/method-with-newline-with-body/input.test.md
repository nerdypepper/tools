# `index.test.ts`

**DO NOT MODIFY**. This file has been autogenerated. Run `rome test packages/@romefrontend/js-parser/index.test.ts --update-snapshots` to update.

## `typescript > class > method-with-newline-with-body`

### `ast`

```javascript
JSRoot {
	comments: Array []
	corrupt: false
	diagnostics: Array []
	directives: Array []
	filename: "input.ts"
	hasHoistedVars: false
	interpreter: undefined
	mtime: undefined
	sourceType: "module"
	syntax: Array ["ts"]
	loc: Object {
		filename: "input.ts"
		end: Object {
			column: 0
			index: 32
			line: 7
		}
		start: Object {
			column: 0
			index: 0
			line: 1
		}
	}
	body: Array [
		JSClassDeclaration {
			id: JSBindingIdentifier {
				name: "C"
				loc: Object {
					filename: "input.ts"
					identifierName: "C"
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
				filename: "input.ts"
				end: Object {
					column: 1
					index: 31
					line: 6
				}
				start: Object {
					column: 0
					index: 0
					line: 1
				}
			}
			meta: JSClassHead {
				implements: undefined
				superClass: undefined
				superTypeParameters: undefined
				typeParameters: undefined
				loc: Object {
					filename: "input.ts"
					end: Object {
						column: 1
						index: 31
						line: 6
					}
					start: Object {
						column: 0
						index: 0
						line: 1
					}
				}
				body: Array [
					JSClassMethod {
						kind: "method"
						key: JSStaticPropertyKey {
							value: JSIdentifier {
								name: "m"
								loc: Object {
									filename: "input.ts"
									identifierName: "m"
									end: Object {
										column: 5
										index: 15
										line: 3
									}
									start: Object {
										column: 4
										index: 14
										line: 3
									}
								}
							}
							loc: Object {
								filename: "input.ts"
								end: Object {
									column: 5
									index: 15
									line: 3
								}
								start: Object {
									column: 4
									index: 14
									line: 3
								}
							}
						}
						loc: Object {
							filename: "input.ts"
							end: Object {
								column: 5
								index: 29
								line: 5
							}
							start: Object {
								column: 4
								index: 14
								line: 3
							}
						}
						body: JSBlockStatement {
							body: Array []
							directives: Array []
							loc: Object {
								filename: "input.ts"
								end: Object {
									column: 5
									index: 29
									line: 5
								}
								start: Object {
									column: 4
									index: 22
									line: 4
								}
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
								filename: "input.ts"
								end: Object {
									column: 7
									index: 17
									line: 3
								}
								start: Object {
									column: 5
									index: 15
									line: 3
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
								column: 4
								index: 14
								line: 3
							}
							loc: Object {
								filename: "input.ts"
								end: Object {
									column: 5
									index: 15
									line: 3
								}
								start: Object {
									column: 4
									index: 14
									line: 3
								}
							}
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
✔ No known problems!

```