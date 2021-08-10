# `index.test.ts`

**DO NOT MODIFY**. This file has been autogenerated. Run `rome test internal/js-parser/index.test.ts --update-snapshots` to update.

## `typescript > class > private-fields-declare`

### `ast`

```javascript
JSRoot {
	body: [
		JSClassDeclaration {
			id: JSBindingIdentifier {
				name: "A"
				loc: SourceLocation typescript/class/private-fields-declare/input.ts 1:6-1:7 (A)
			}
			meta: JSClassHead {
				body: [
					JSClassPrivateProperty {
						key: JSPrivateName {
							id: JSIdentifier {
								name: "x"
								loc: SourceLocation typescript/class/private-fields-declare/input.ts 2:11-2:12 (x)
							}
							loc: SourceLocation typescript/class/private-fields-declare/input.ts 2:10-2:12
						}
						meta: JSClassPropertyMeta {
							abstract: false
							declare: true
							optional: false
							readonly: false
							static: false
							loc: SourceLocation typescript/class/private-fields-declare/input.ts 2:2-2:12
							start: Position 2:2
						}
						loc: SourceLocation typescript/class/private-fields-declare/input.ts 2:2-2:13
					}
					JSClassPrivateProperty {
						key: JSPrivateName {
							id: JSIdentifier {
								name: "y"
								loc: SourceLocation typescript/class/private-fields-declare/input.ts 3:11-3:12 (y)
							}
							loc: SourceLocation typescript/class/private-fields-declare/input.ts 3:10-3:12
						}
						meta: JSClassPropertyMeta {
							abstract: false
							declare: true
							optional: false
							readonly: false
							static: false
							loc: SourceLocation typescript/class/private-fields-declare/input.ts 3:2-3:12
							start: Position 3:2
						}
						typeAnnotation: TSStringKeywordTypeAnnotation {
							loc: SourceLocation typescript/class/private-fields-declare/input.ts 3:14-3:20
						}
						loc: SourceLocation typescript/class/private-fields-declare/input.ts 3:2-3:21
					}
				]
				loc: SourceLocation typescript/class/private-fields-declare/input.ts 1:0-4:1
			}
			loc: SourceLocation typescript/class/private-fields-declare/input.ts 1:0-4:1
		}
	]
	comments: []
	corrupt: false
	diagnostics: [
		{
			origins: [{entity: "ParserCore<js>"}]
			description: {
				advice: []
				category: ["parse"]
				categoryValue: "js"
				message: RAW_MARKUP {value: "'declare' modifier cannot be used with a private field."}
			}
			location: {
				language: "js"
				path: UIDPath<typescript/class/private-fields-declare/input.ts>
				end: Position 2:12
				start: Position 2:10
			}
		}
	]
	directives: []
	hasHoistedVars: false
	sourceType: "module"
	syntax: ["ts"]
	path: UIDPath<typescript/class/private-fields-declare/input.ts>
	loc: SourceLocation typescript/class/private-fields-declare/input.ts 1:0-5:0
}
```

### `diagnostics`

```

 typescript/class/private-fields-declare/input.ts:2:10 parse(js) ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  ✖ 'declare' modifier cannot be used with a private field.

    1 │ class A {
  > 2 │   declare #x;
      │           ^^
    3 │   declare #y: string;
    4 │ }


```