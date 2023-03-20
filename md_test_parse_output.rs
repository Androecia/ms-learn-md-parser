MsMarkdown {
    tokens: [
        Metadata(
            MsMdMetadata {
                author: "meganbradley",
                description: Some(
                    "Learn the Markdown features and syntax used in Microsoft Learn content.",
                ),
                ms_author: "mbradley",
                ms_date: None,
                ms_service: None,
                ms_prod: Some(
                    "non-product-specific",
                ),
                ms_topic: Some(
                    "contributor-guide",
                ),
                title: "Markdown reference for Microsoft Learn",
                ms_custom: Some(
                    "external-contributor-guide",
                ),
                ms_reviewer: None,
                ms_subservice: None,
                technology: None,
                robots: None,
                no_loc: None,
            },
        ),
        LineBreak,
        Heading {
            level: 1,
            text: "Learn Markdown reference",
        },
        LineBreak,
        TextBlock {
            indent: 0,
            content: "This article provides an alphabetical reference for writing Markdown for [Microsoft Learn](/).",
        },
        LineBreak,
        TextBlock {
            indent: 0,
            content: "[Markdown](https://daringfireball.net/projects/markdown/) is a lightweight markup language with plain text formatting syntax. The Microsoft Learn platform supports [CommonMark](https://commonmark.org/) compliant Markdown parsed through the [Markdig](https://github.com/lunet-io/markdig) parsing engine. Microsoft Learn also supports custom Markdown extensions that provide richer content on the Microsoft Learn site.",
        },
        LineBreak,
        TextBlock {
            indent: 0,
            content: "You can use any text editor to write Markdown, but we recommend [Visual Studio Code](https://code.visualstudio.com/) with the [Learn Authoring Pack](https://aka.ms/DocsAuthoringPack). The Learn Authoring Pack provides editing tools and preview functionality that lets you see what your articles will look like when rendered on Microsoft Learn.",
        },
        LineBreak,
        Heading {
            level: 2,
            text: "Alerts (Note, Tip, Important, Caution, Warning)",
        },
        LineBreak,
        TextBlock {
            indent: 0,
            content: "Alerts are a Markdown extension to create block quotes that render on Microsoft Learn with colors and icons that indicate the significance of the content.",
        },
        LineBreak,
        TextBlock {
            indent: 0,
            content: "Avoid notes, tips, and important boxes. Readers tend to skip over them. It's better to put that info directly into the article text.",
        },
        LineBreak,
        TextBlock {
            indent: 0,
            content: "If you need to use alerts, limit them to one or two per article. Multiple notes should never be next to each other in an article.",
        },
        LineBreak,
        TextBlock {
            indent: 0,
            content: "The following alert types are supported:",
        },
        LineBreak,
        Code(
            MsMarkdownCode {
                content: "> [!NOTE]\n> Information the user should notice even if skimming.\n\n> [!TIP]\n> Optional information to help a user be more successful.\n\n> [!IMPORTANT]\n> Essential information required for user success.\n\n> [!CAUTION]\n> Negative potential consequences of an action.\n\n> [!WARNING]\n> Dangerous certain consequences of an action.",
                language: Markdown,
                source: None,
                range: None,
                id: None,
                highlight: None,
                interactive: None,
                indent: 0,
            },
        ),
        TextBlock {
            indent: 0,
            content: "These alerts look like this on Microsoft Learn:",
        },
        LineBreak,
        Alert {
            indent: 0,
            content: "Information the user should notice even if skimming.",
            alert_type: Note,
        },
        Alert {
            indent: 0,
            content: "Optional information to help a user be more successful.",
            alert_type: Tip,
        },
        Alert {
            indent: 0,
            content: "Essential information required for user success.",
            alert_type: Important,
        },
        Alert {
            indent: 0,
            content: "Negative potential consequences of an action.",
            alert_type: Caution,
        },
        Alert {
            indent: 0,
            content: "Dangerous certain consequences of an action.",
            alert_type: Warning,
        },
        Heading {
            level: 2,
            text: "Angle brackets",
        },
        LineBreak,
        TextBlock {
            indent: 0,
            content: "If you use angle brackets in text in your file (for example, to denote a placeholder), you need to manually encode the angle brackets. Otherwise, Markdown thinks that they're intended to be an HTML tag.",
        },
        LineBreak,
        TextBlock {
            indent: 0,
            content: "For example, encode `<script name>` as `&lt;script name&gt;` or `\\<script name>`.",
        },
        LineBreak,
        TextBlock {
            indent: 0,
            content: "Angle brackets don't have to be escaped in text formatted as inline code or in code blocks.",
        },
        LineBreak,
        Heading {
            level: 2,
            text: "Apostrophes and quotation marks",
        },
        LineBreak,
        TextBlock {
            indent: 0,
            content: "If you copy from Word into a Markdown editor, the text might contain \"smart\" (curly) apostrophes or quotation marks. These need to be encoded or changed to basic apostrophes or quotation marks. Otherwise, you end up with things like this when the file is published: Itâ&euro;&trade;s",
        },
        LineBreak,
        TextBlock {
            indent: 0,
            content: "Here are the encodings for the \"smart\" versions of these punctuation marks:",
        },
        LineBreak,
        List(
            [
                MdListItem {
                    indent: 0,
                    list_type: Unordered,
                    content: "Left (opening) quotation mark: `&#8220;`",
                },
                MdListItem {
                    indent: 0,
                    list_type: Unordered,
                    content: "Right (closing) quotation mark: `&#8221;`",
                },
                MdListItem {
                    indent: 0,
                    list_type: Unordered,
                    content: "Right (closing) single quotation mark or apostrophe: `&#8217;`",
                },
                MdListItem {
                    indent: 0,
                    list_type: Unordered,
                    content: "Left (opening) single quotation mark (rarely used): `&#8216;`",
                },
            ],
        ),
        Alert {
            indent: 0,
            content: "To avoid \"smart\" characters in your Markdown files, rely on the Learn Authoring Pack's smart quote replacement feature. For more information, see [smart quote replacement](docs-authoring/smart-quote-replacement.md).",
            alert_type: Tip,
        },
        Heading {
            level: 2,
            text: "Blockquotes",
        },
        LineBreak,
        TextBlock {
            indent: 0,
            content: "Blockquotes are created using the `>` character:",
        },
        LineBreak,
        Code(
            MsMarkdownCode {
                content: "> This is a blockquote. It is usually rendered indented and with a different background color.",
                language: Markdown,
                source: None,
                range: None,
                id: None,
                highlight: None,
                interactive: None,
                indent: 0,
            },
        ),
        TextBlock {
            indent: 0,
            content: "The preceding example renders as follows:",
        },
        LineBreak,
        Quote {
            indent: 0,
            content: "This is a blockquote. It is usually rendered indented and with a different background color.",
        },
        LineBreak,
        Heading {
            level: 2,
            text: "Bold and italic text",
        },
        LineBreak,
        TextBlock {
            indent: 0,
            content: "To format text as **bold**, enclose it in two asterisks:",
        },
        LineBreak,
        Code(
            MsMarkdownCode {
                content: "This text is **bold**.",
                language: Markdown,
                source: None,
                range: None,
                id: None,
                highlight: None,
                interactive: None,
                indent: 0,
            },
        ),
        TextBlock {
            indent: 0,
            content: "To format text as *italic*, enclose it in a single asterisk:",
        },
        LineBreak,
        Code(
            MsMarkdownCode {
                content: "This text is *italic*.",
                language: Markdown,
                source: None,
                range: None,
                id: None,
                highlight: None,
                interactive: None,
                indent: 0,
            },
        ),
        TextBlock {
            indent: 0,
            content: "To format text as both ***bold and italic***, enclose it in three asterisks:",
        },
        LineBreak,
        Code(
            MsMarkdownCode {
                content: "This text is both ***bold and italic***.",
                language: Markdown,
                source: None,
                range: None,
                id: None,
                highlight: None,
                interactive: None,
                indent: 0,
            },
        ),
        TextBlock {
            indent: 0,
            content: "For guidance on when to use bold and italic text, see [text formatting guidelines](text-formatting-guidelines.md).",
        },
        LineBreak,
        Heading {
            level: 2,
            text: "Code snippets",
        },
        LineBreak,
        TextBlock {
            indent: 0,
            content: "Learn Markdown supports the placement of code snippets both inline in a sentence and as a separate \"fenced\" block between sentences. For more information, see [How to add code to docs](code-in-docs.md).",
        },
        LineBreak,
        Heading {
            level: 2,
            text: "Columns",
        },
        LineBreak,
        TextBlock {
            indent: 0,
            content: "The **columns** Markdown extension gives authors the ability to add column-based content layouts that are more flexible and powerful than basic Markdown tables, which are only suited for true tabular data. You can add up to four columns, and use the optional `span` attribute to merge two or more columns.",
        },
        LineBreak,
        TextBlock {
            indent: 0,
            content: "While the **columns** extension still works, we no longer recommend it for creating custom layouts. We've found that many custom column layouts have accessibility issues or otherwise violate the style guidelines. Don't create custom layouts. Use standard Microsoft Learn features.",
        },
        LineBreak,
        TextBlock {
            indent: 0,
            content: "The syntax for columns is as follows:",
        },
        LineBreak,
        Code(
            MsMarkdownCode {
                content: ":::row:::\n   :::column span=\"\":::\n      Content...\n   :::column-end:::\n   :::column span=\"\":::\n      More content...\n   :::column-end:::\n:::row-end:::",
                language: Markdown,
                source: None,
                range: None,
                id: None,
                highlight: None,
                interactive: None,
                indent: 0,
            },
        ),
        TextBlock {
            indent: 0,
            content: "Columns should only contain basic Markdown, including images. Headings, tables, tabs, and other complex structures shouldn't be included. A row can't have any content outside of column.",
        },
        LineBreak,
        TextBlock {
            indent: 0,
            content: "For example, the following Markdown creates one column that spans two column widths, and one standard (no `span`) column:",
        },
        LineBreak,
        Code(
            MsMarkdownCode {
                content: ":::row:::\n   :::column span=\"2\":::\n      **This is a 2-span column with lots of text.**\n\n      Lorem ipsum dolor sit amet, consectetur adipiscing elit. Donec vestibulum mollis nunc\n      ornare commodo. Nullam ac metus imperdiet, rutrum justo vel, vulputate leo. Donec\n      rutrum non eros eget consectetur.\n   :::column-end:::\n   :::column span=\"\":::\n      **This is a single-span column with an image in it.**\n\n      ![Doc.U.Ment](media/markdown-reference/document.png)\n   :::column-end:::\n:::row-end:::",
                language: Markdown,
                source: None,
                range: None,
                id: None,
                highlight: None,
                interactive: None,
                indent: 0,
            },
        ),
        TextBlock {
            indent: 0,
            content: "This renders as follows:",
        },
        LineBreak,
        Row(
            [
                MsMdColumn {
                    span: Some(
                        2,
                    ),
                    content: "\n**This is a 2-span column with lots of text.**\n\nLorem ipsum dolor sit amet, consectetur adipiscing elit. Donec vestibulum mollis nunc\nornare commodo. Nullam ac metus imperdiet, rutrum justo vel, vulputate leo. Donec\nrutrum non eros eget consectetur.\n",
                },
                MsMdColumn {
                    span: None,
                    content: "\n**This is a single-span column with an image in it.**\n\n![Doc.U.Ment](media/markdown-reference/document.png)\n",
                },
            ],
        ),
        LineBreak,
        Heading {
            level: 2,
            text: "Comments",
        },
        LineBreak,
        TextBlock {
            indent: 0,
            content: "Microsoft Learn supports HTML comments if you must comment out sections of your article:",
        },
        LineBreak,
        Code(
            MsMarkdownCode {
                content: "<!--- Here's my comment --->",
                language: Html,
                source: None,
                range: None,
                id: None,
                highlight: None,
                interactive: None,
                indent: 0,
            },
        ),
        Alert {
            indent: 0,
            content: "Do not put private or sensitive information in HTML comments. Microsoft Learn carries HTML comments through to the published HTML that goes public. While HTML comments are invisible to the reader's eye, they are exposed in the HTML underneath.",
            alert_type: Warning,
        },
        Heading {
            level: 2,
            text: "Headings",
        },
        LineBreak,
        TextBlock {
            indent: 0,
            content: "Microsoft Learn supports six levels of Markdown headings:",
        },
        LineBreak,
        Code(
            MsMarkdownCode {
                content: "# This is a first level heading (H1)\n\n## This is a second level heading (H2)\n\n...\n\n###### This is a sixth level heading (H6)",
                language: Markdown,
                source: None,
                range: None,
                id: None,
                highlight: None,
                interactive: None,
                indent: 0,
            },
        ),
        List(
            [
                MdListItem {
                    indent: 0,
                    list_type: Unordered,
                    content: "There must be a space between the last `#` and heading text.",
                },
                MdListItem {
                    indent: 0,
                    list_type: Unordered,
                    content: "Each Markdown file must have one and only one H1 heading.",
                },
                MdListItem {
                    indent: 0,
                    list_type: Unordered,
                    content: "The H1 heading must be the first content in the file after the YML metadata block.",
                },
                MdListItem {
                    indent: 0,
                    list_type: Unordered,
                    content: "H2 headings automatically appear in the right-hand navigating menu of the published file. Lower-level headings don't appear, so use H2s strategically to help readers navigate your content.",
                },
                MdListItem {
                    indent: 0,
                    list_type: Unordered,
                    content: "HTML headings, such as `<h1>`, aren't recommended, and in some cases will cause build warnings.",
                },
                MdListItem {
                    indent: 0,
                    list_type: Unordered,
                    content: "You can link to individual headings in a file via [bookmark links](how-to-write-links.md#explicit-anchor-links).",
                },
            ],
        ),
        Heading {
            level: 2,
            text: "HTML",
        },
        LineBreak,
        TextBlock {
            indent: 0,
            content: "Although Markdown supports inline HTML, HTML isn't recommended for publishing to Microsoft Learn, and except for a limited list of values will cause build errors or warnings.",
        },
        LineBreak,
        Heading {
            level: 2,
            text: "Images",
        },
        LineBreak,
        TextBlock {
            indent: 0,
            content: "The following file types are supported by default for images:",
        },
        LineBreak,
        List(
            [
                MdListItem {
                    indent: 0,
                    list_type: Unordered,
                    content: ".jpg",
                },
                MdListItem {
                    indent: 0,
                    list_type: Unordered,
                    content: ".png",
                },
            ],
        ),
        TextBlock {
            indent: 0,
            content: "To support other image types, such as .gif, you must add them as resources in *docfx.json*:",
        },
        LineBreak,
        Code(
            MsMarkdownCode {
                content: "\"resource\": [\n  {\n    \"files\" : [\n      \"**/*.png\",\n      \"**/*.jpg,\n      \"**/*.gif\"\n    ],",
                language: Markdown,
                source: None,
                range: None,
                id: None,
                highlight: None,
                interactive: None,
                indent: 0,
            },
        ),
        Heading {
            level: 3,
            text: "Standard conceptual images (default Markdown)",
        },
        LineBreak,
        TextBlock {
            indent: 0,
            content: "The basic Markdown syntax to embed an image is:",
        },
        LineBreak,
        Code(
            MsMarkdownCode {
                content: "![<alt text>](<folderPath>)\n\nExample:\n![alt text for image](../images/Introduction.png)",
                language: Markdown,
                source: None,
                range: None,
                id: None,
                highlight: None,
                interactive: None,
                indent: 0,
            },
        ),
        TextBlock {
            indent: 0,
            content: "Where `<alt text>` is a brief description of the image and `<folder path>` is a relative path to the image. Alternate text is required for screen readers for the visually impaired. It's also useful if there's a site bug where the image can't render.",
        },
        LineBreak,
        TextBlock {
            indent: 0,
            content: "Underscores in alt text aren't rendered properly unless you escape them by prefixing them with a backslash (`\\_`). However, don't copy file names for use as alt text. For example, instead of this:",
        },
        LineBreak,
        Code(
            MsMarkdownCode {
                content: "![ADextension_2FA_Configure_Step4](./media/bogusfilename/ADextension_2FA_Configure_Step4.PNG)",
                language: Markdown,
                source: None,
                range: None,
                id: None,
                highlight: None,
                interactive: None,
                indent: 0,
            },
        ),
        TextBlock {
            indent: 0,
            content: "Write this:",
        },
        LineBreak,
        Code(
            MsMarkdownCode {
                content: "![Active Directory extension for two-factor authentication, step 4: Configure](./media/bogusfilename/ADextension_2FA_Configure_Step4.PNG)",
                language: Markdown,
                source: None,
                range: None,
                id: None,
                highlight: None,
                interactive: None,
                indent: 0,
            },
        ),
        Heading {
            level: 3,
            text: "Standard conceptual images (Learn Markdown)",
        },
        LineBreak,
        TextBlock {
            indent: 0,
            content: "The custom `:::image:::` extension on Microsoft Learn supports standard images, complex images, and icons.",
        },
        LineBreak,
        TextBlock {
            indent: 0,
            content: "For standard images, the older Markdown syntax will still work, but the new extension is recommended because it supports more powerful functionality, such as specifying a localization scope that's different from the parent topic. Other advanced functionality, such as selecting from the shared image gallery instead of specifying a local image, will be available in the future. The new syntax is as follows:",
        },
        LineBreak,
        Code(
            MsMarkdownCode {
                content: ":::image type=\"content\" source=\"<folderPath>\" alt-text=\"<alt text>\":::",
                language: Markdown,
                source: None,
                range: None,
                id: None,
                highlight: None,
                interactive: None,
                indent: 0,
            },
        ),
        LineBreak,
        TextBlock {
            indent: 0,
            content: "If `type=\"content\"` (the default), both `source` and `alt-text` are required.",
        },
        LineBreak,
        Heading {
            level: 3,
            text: "Complex images with long descriptions",
        },
        LineBreak,
        TextBlock {
            indent: 0,
            content: "You can also use this extension to add an image with a long description that is read by screen readers but not rendered visually on the published page. Long descriptions are an accessibility requirement for complex images, such as graphs. The syntax is the following:",
        },
        LineBreak,
        Code(
            MsMarkdownCode {
                content: ":::image type=\"complex\" source=\"<folderPath>\" alt-text=\"<alt text>\":::\n   <long description here>\n:::image-end:::",
                language: Markdown,
                source: None,
                range: None,
                id: None,
                highlight: None,
                interactive: None,
                indent: 0,
            },
        ),
        LineBreak,
        TextBlock {
            indent: 0,
            content: "If `type=\"complex\"`, `source`, `alt-text`, a long description, and the `:::image-end:::` tag are all required.",
        },
        LineBreak,
        TextBlock {
            indent: 0,
            content: "When your changes are in preview or published, you can check whether the long description exists by right-clicking on the image and selecting **Inspect** (when using Microsoft Edge browser, although other browsers have similar features). This action brings you to the image source in the HTML code, underneath which you'll find a *visually-hidden* class. Expand the dropdown on this class, and you'll find your long description:",
        },
        LineBreak,
        LineBreak,
        Heading {
            level: 3,
            text: "Automatic borders",
        },
        LineBreak,
        TextBlock {
            indent: 0,
            content: "The `:::image:::` extension also supports the `border` property, which  automatically adds a 1-pixel gray border around your image. The `border` property is `true` by default for `content` and `complex` images, so you'll get the border automatically unless you explicitly add the property with a value of `false`. The `border` property is `false` by default for `icon` images.",
        },
        LineBreak,
        TextBlock {
            indent: 0,
            content: "The `border` property is the recommended way to add a border. Don't create your own borders manually.",
        },
        LineBreak,
        Comment(
            " This section can be allowed publicly, but there's no external guide article for how to use lightboxes, so we can't add it until we have an external-guide equivalent.\n\n### Creating an expandable image\n\nThe optional `lightbox` property allows you to create an expanded image, as described in [Create an expandable screenshot (lightbox)](contribute-how-to-use-lightboxes.md). The value of `lightbox` is the path to the expanded image.\n\n",
        ),
        LineBreak,
        Heading {
            level: 3,
            text: "Specifying loc-scope",
        },
        LineBreak,
        TextBlock {
            indent: 0,
            content: "Sometimes the localization scope for an image is different from that of the article or module that contains it. This can cause a bad global experience: for example, if a screenshot of a product is accidentally localized into a language the product isn't available in. To prevent this, you can specify the optional `loc-scope` attribute in images of types `content` and `complex`, and is required for screenshots that show a product with a different localization scope than the article or module that contains it.",
        },
        LineBreak,
        Heading {
            level: 3,
            text: "Icons",
        },
        LineBreak,
        TextBlock {
            indent: 0,
            content: "The image extension supports icons, which are decorative images and should not have alt text. The syntax for icons is:",
        },
        LineBreak,
        Code(
            MsMarkdownCode {
                content: ":::image type=\"icon\" source=\"<folderPath>\":::",
                language: Markdown,
                source: None,
                range: None,
                id: None,
                highlight: None,
                interactive: None,
                indent: 0,
            },
        ),
        TextBlock {
            indent: 0,
            content: "If `type=\"icon\"`, `source` should be specified but `alt-text` shouldn't be.",
        },
        LineBreak,
        TextBlock {
            indent: 0,
            content: "The `border` property  is `false` by default for icons. If your decorative image requires the standard image border, explicitly add `border=\"true\"` to the `:::image:::` tag.",
        },
        LineBreak,
        Comment(
            " No lightbox article in external guide, so commenting this out for now.\n\nThe `lightbox` property works the same for `icon` images as for standard `content` images.\n\n",
        ),
        LineBreak,
        Heading {
            level: 2,
            text: "Included Markdown files",
        },
        LineBreak,
        TextBlock {
            indent: 0,
            content: "Where markdown files need to be repeated in multiple articles, you can use an include file. The includes feature instructs Microsoft Learn to replace the reference with the contents of the include file at build time. You can use includes in the following ways:",
        },
        LineBreak,
        List(
            [
                MdListItem {
                    indent: 0,
                    list_type: Unordered,
                    content: "Inline: Reuse a common text snippet inline with within a sentence.",
                },
                MdListItem {
                    indent: 0,
                    list_type: Unordered,
                    content: "Block: Reuse an entire Markdown file as a block, nested within a section of an article.",
                },
            ],
        ),
        TextBlock {
            indent: 0,
            content: "An inline or block include file is a Markdown (.md) file. It can contain any valid Markdown. Include files are typically located in a common *includes* subdirectory, in the root of the repository. When the article is published, the included file is seamlessly integrated into it.",
        },
        LineBreak,
        Heading {
            level: 3,
            text: "Includes syntax",
        },
        LineBreak,
        TextBlock {
            indent: 0,
            content: "Block include is on its own line:",
        },
        LineBreak,
        Code(
            MsMarkdownCode {
                content: "[!INCLUDE [<title>](<filepath>)]",
                language: Markdown,
                source: None,
                range: None,
                id: None,
                highlight: None,
                interactive: None,
                indent: 0,
            },
        ),
        TextBlock {
            indent: 0,
            content: "Inline include is within a line:",
        },
        LineBreak,
        Code(
            MsMarkdownCode {
                content: "Text before [!INCLUDE [<title>](<filepath>)] and after.",
                language: Markdown,
                source: None,
                range: None,
                id: None,
                highlight: None,
                interactive: None,
                indent: 0,
            },
        ),
        TextBlock {
            indent: 0,
            content: "Where `<title>` is the name of the file and `<filepath>` is the relative path to the file. `INCLUDE` must be capitalized and there must be a space before the `<title>`.",
        },
        LineBreak,
        TextBlock {
            indent: 0,
            content: "Here are requirements and considerations for include files:",
        },
        LineBreak,
        List(
            [
                MdListItem {
                    indent: 0,
                    list_type: Unordered,
                    content: "Use block includes for significant amounts of content--a paragraph or two, a shared procedure, or a shared section. Don't use them for anything smaller than a sentence.",
                },
                MdListItem {
                    indent: 0,
                    list_type: Unordered,
                    content: "Includes won't be rendered in the GitHub-rendered view of your article because they rely on Microsoft Learn extensions. They'll be rendered only after publication.",
                },
                MdListItem {
                    indent: 0,
                    list_type: Unordered,
                    content: "Write all the text in an include file in complete sentences or phrases that don't depend on preceding or following text in the article that references the include. Ignoring this guidance creates an untranslatable string in the article.",
                },
                MdListItem {
                    indent: 0,
                    list_type: Unordered,
                    content: "Don't embed include files within other include files.",
                },
                MdListItem {
                    indent: 0,
                    list_type: Unordered,
                    content: "`/Includes` folders are excluded from build. Therefore, images stored in `/includes` folders and referenced in included files won't be displayed in published content. Store images in a `/media` folder outside the `/includes` folder.",
                },
                MdListItem {
                    indent: 0,
                    list_type: Unordered,
                    content: "As with regular articles, don't share media between include files. Use a separate file with a unique name for each include and article. Store the media file in the media folder that's associated with the include.",
                },
                MdListItem {
                    indent: 0,
                    list_type: Unordered,
                    content: "Don't use an include as the only content of an article.  Includes are meant to be supplemental to the content in the rest of the article.",
                },
            ],
        ),
        Heading {
            level: 2,
            text: "Indentation",
        },
        LineBreak,
        TextBlock {
            indent: 0,
            content: "In Markdown, spaces before the first character of a line determine the line's alignment relative to the preceding lines. Indentation especially influences numbered and bulleted lists to render multiple levels of nesting in a hierarchical or outline format.",
        },
        LineBreak,
        TextBlock {
            indent: 0,
            content: "To indent text to align with a preceding paragraph or an item in a numbered or bulleted list, use spaces.",
        },
        LineBreak,
        TextBlock {
            indent: 0,
            content: "The following two examples show how indented paragraphs render based on their relationship to other paragraphs.",
        },
        LineBreak,
        Code(
            MsMarkdownCode {
                content: "1. This is a numbered list example (one space after the period before the letter T).\n   This sentence is indented three spaces.\n   This code block is indented three spaces.\n\n- This is a bulleted list example (one space after the bullet before the letter T).\n  This sentence is indented two spaces.\n  > [!TIP]\n  > This tip is indented two spaces.\n  - This is a second-level bullet (indented two spaces, with one space after the bullet before the letter T).\n    This sentence is indented four spaces.\n    > This quote block is indented four spaces.",
                language: None,
                source: None,
                range: None,
                id: None,
                highlight: None,
                interactive: None,
                indent: 0,
            },
        ),
        TextBlock {
            indent: 0,
            content: "The example above renders as:",
        },
        LineBreak,
        List(
            [
                MdListItem {
                    indent: 0,
                    list_type: Ordered,
                    content: "This is a numbered list example (one space after the period before the letter T).",
                },
            ],
        ),
        TextBlock {
            indent: 3,
            content: "This sentence is indented three spaces.",
        },
        LineBreak,
        Code(
            MsMarkdownCode {
                content: "This code block is indented three spaces.\n",
                language: None,
                source: None,
                range: None,
                id: None,
                highlight: None,
                interactive: None,
                indent: 3,
            },
        ),
        List(
            [
                MdListItem {
                    indent: 0,
                    list_type: Unordered,
                    content: "This is a bulleted list example (one space after the bullet before the letter T).",
                },
            ],
        ),
        TextBlock {
            indent: 2,
            content: "This sentence is indented two spaces.",
        },
        LineBreak,
        Alert {
            indent: 2,
            content: "This tip is indented two spaces.",
            alert_type: Tip,
        },
        List(
            [
                MdListItem {
                    indent: 2,
                    list_type: Unordered,
                    content: "This is a second-level bullet (indented two spaces, with one space after the bullet before the letter T).",
                },
            ],
        ),
        TextBlock {
            indent: 4,
            content: "This sentence is indented four spaces.",
        },
        LineBreak,
        Quote {
            indent: 4,
            content: "> This quote block is indented four spaces.",
        },
        LineBreak,
        Heading {
            level: 2,
            text: "Links",
        },
        LineBreak,
        TextBlock {
            indent: 0,
            content: "For information on syntax for links, see [Use links in documentation](how-to-write-links.md).",
        },
        LineBreak,
        Heading {
            level: 2,
            text: "Lists (Numbered, Bulleted, Checklist)",
        },
        LineBreak,
        Heading {
            level: 3,
            text: "Numbered list",
        },
        LineBreak,
        TextBlock {
            indent: 0,
            content: "To create a numbered list, you can use all 1s. The numbers are rendered in ascending order as a sequential list when published. For increased source readability, you can increment your lists manually.",
        },
        LineBreak,
        TextBlock {
            indent: 0,
            content: "Don't use letters in lists, including nested lists. They don't render correctly when published to Microsoft Learn. Nested lists using numbers will render as lowercase letters when published. For example:",
        },
        LineBreak,
        Code(
            MsMarkdownCode {
                content: "1. This is\n1. a parent numbered list\n   1. and this is\n   1. a nested numbered list\n1. (fin)",
                language: Markdown,
                source: None,
                range: None,
                id: None,
                highlight: None,
                interactive: None,
                indent: 0,
            },
        ),
        TextBlock {
            indent: 0,
            content: "This renders as follows:",
        },
        LineBreak,
        List(
            [
                MdListItem {
                    indent: 0,
                    list_type: Ordered,
                    content: "This is",
                },
                MdListItem {
                    indent: 0,
                    list_type: Ordered,
                    content: "a parent numbered list",
                },
                MdListItem {
                    indent: 3,
                    list_type: Unordered,
                    content: "and this is",
                },
                MdListItem {
                    indent: 3,
                    list_type: Unordered,
                    content: "a nested numbered list",
                },
                MdListItem {
                    indent: 0,
                    list_type: Ordered,
                    content: "(fin)",
                },
            ],
        ),
        Heading {
            level: 3,
            text: "Bulleted list",
        },
        LineBreak,
        TextBlock {
            indent: 0,
            content: "To create a bulleted list, use `-` or `*` followed by a space at the beginning of each line:",
        },
        LineBreak,
        Code(
            MsMarkdownCode {
                content: "- This is\n- a parent bulleted list\n  - and this is\n  - a nested bulleted list\n- All done!",
                language: Markdown,
                source: None,
                range: None,
                id: None,
                highlight: None,
                interactive: None,
                indent: 0,
            },
        ),
        TextBlock {
            indent: 0,
            content: "This renders as follows:",
        },
        LineBreak,
        List(
            [
                MdListItem {
                    indent: 0,
                    list_type: Unordered,
                    content: "This is",
                },
                MdListItem {
                    indent: 0,
                    list_type: Unordered,
                    content: "a parent bulleted list",
                },
                MdListItem {
                    indent: 2,
                    list_type: Unordered,
                    content: "and this is",
                },
                MdListItem {
                    indent: 2,
                    list_type: Unordered,
                    content: "a nested bulleted list",
                },
                MdListItem {
                    indent: 0,
                    list_type: Unordered,
                    content: "All done!",
                },
            ],
        ),
        TextBlock {
            indent: 0,
            content: "Whichever syntax you use, `-` or `*`, use it consistently within an article.",
        },
        LineBreak,
        Heading {
            level: 3,
            text: "Checklist",
        },
        LineBreak,
        TextBlock {
            indent: 0,
            content: "Checklists are available for use on Microsoft Learn via a custom Markdown extension:",
        },
        LineBreak,
        Code(
            MsMarkdownCode {
                content: "> [!div class=\"checklist\"]\n> * List item 1\n> * List item 2\n> * List item 3",
                language: Markdown,
                source: None,
                range: None,
                id: None,
                highlight: None,
                interactive: None,
                indent: 0,
            },
        ),
        TextBlock {
            indent: 0,
            content: "This example renders on Microsoft Learn like this:",
        },
        LineBreak,
        TextBlock {
            indent: 0,
            content: "Use checklists at the beginning or end of an article to summarize \"What will you learn\" or \"What have you learned\" content. Do not add random checklists throughout your articles.",
        },
        LineBreak,
        Heading {
            level: 2,
            text: "Next step action",
        },
        LineBreak,
        TextBlock {
            indent: 0,
            content: "You can use a custom extension to add a next step action button to Microsoft Learn pages.",
        },
        LineBreak,
        TextBlock {
            indent: 0,
            content: "The syntax is as follows:",
        },
        LineBreak,
        Code(
            MsMarkdownCode {
                content: "> [!div class=\"nextstepaction\"]\n> [button text](link to topic)",
                language: Markdown,
                source: None,
                range: None,
                id: None,
                highlight: None,
                interactive: None,
                indent: 0,
            },
        ),
        TextBlock {
            indent: 0,
            content: "For example:",
        },
        LineBreak,
        Code(
            MsMarkdownCode {
                content: "> [!div class=\"nextstepaction\"]\n> [Learn about adding code to articles](code-in-docs.md)",
                language: Markdown,
                source: None,
                range: None,
                id: None,
                highlight: None,
                interactive: None,
                indent: 0,
            },
        ),
        TextBlock {
            indent: 0,
            content: "This renders as follows:",
        },
        LineBreak,
        TextBlock {
            indent: 0,
            content: "You can use any supported link in a next step action, including a Markdown link to another web page. In most cases, the next action link will be a relative link to another file in the same docset.",
        },
        LineBreak,
        Heading {
            level: 2,
            text: "Non-localized strings",
        },
        LineBreak,
        TextBlock {
            indent: 0,
            content: "You can use the custom `no-loc` Markdown extension to identify strings of content that you would like the localization process to ignore.",
        },
        LineBreak,
        TextBlock {
            indent: 0,
            content: "All strings called out will be case-sensitive; that is, the string must match exactly to be ignored for localization.",
        },
        LineBreak,
        TextBlock {
            indent: 0,
            content: "To mark an individual string as non-localizable, use this syntax:",
        },
        LineBreak,
        Code(
            MsMarkdownCode {
                content: ":::no-loc text=\"String\":::",
                language: Markdown,
                source: None,
                range: None,
                id: None,
                highlight: None,
                interactive: None,
                indent: 0,
            },
        ),
        TextBlock {
            indent: 0,
            content: "For example, in the following, only the single instance of `Document` will be ignored during the localization process:",
        },
        LineBreak,
        Code(
            MsMarkdownCode {
                content: "# Heading 1 of the Document\n\nMarkdown content within the :::no-loc text=\"Document\":::.  The are multiple instances of Document, document, and documents.",
                language: Markdown,
                source: None,
                range: None,
                id: None,
                highlight: None,
                interactive: None,
                indent: 0,
            },
        ),
        TextBlock {
            indent: 0,
            content: "```",
        },
        LineBreak,
        TextBlock {
            indent: 0,
            content: "You can also use metadata in the YAML header to mark all instances of a string within the current Markdown file as non-localizable:",
        },
        LineBreak,
        Code(
            MsMarkdownCode {
                content: "author: cillroy\nno-loc: [Global, Strings, to be, Ignored]",
                language: YAML,
                source: None,
                range: None,
                id: None,
                highlight: None,
                interactive: None,
                indent: 0,
            },
        ),
        Alert {
            indent: 0,
            content: "The no-loc metadata is not supported as global metadata in *docfx.json* file. The localization pipeline doesn't read the *docfx.json* file, so the no-loc metadata must be added into each individual source file.",
            alert_type: Note,
        },
        TextBlock {
            indent: 0,
            content: "In the following example, both in the metadata `title` and the Markdown header the word `Document` will be ignored during the localization process.",
        },
        LineBreak,
        TextBlock {
            indent: 0,
            content: "In the metadata `description` and the Markdown main content the word `document` is localized, because it does not start with a capital `D`.",
        },
        LineBreak,
        Code(
            MsMarkdownCode {
                content: "---\ntitle: Title of the Document\nauthor: author-name\ndescription: Description for the document\nno-loc: [Title, Document]\n---\n# Heading 1 of the Document\n\nMarkdown content within the document.",
                language: Markdown,
                source: None,
                range: None,
                id: None,
                highlight: None,
                interactive: None,
                indent: 0,
            },
        ),
        Comment(
            " commenting out for now because no one knows what this means\n## Section definition\n\nYou might need to define a section. This syntax is mostly used for code tables.\nSee the following example:\n\n````\n> [!div class=\"tabbedCodeSnippets\" data-resources=\"OutlookServices.Calendar\"]\n> ```cs\n> <cs code text>\n> ```\n> ```javascript\n> <js code text>\n> ```\n````\n\nThe preceding blockquote Markdown text will be rendered as:\n> [!div class=\"tabbedCodeSnippets\" data-resources=\"OutlookServices.Calendar\"]\n> ```cs\n> <cs code text>\n> ```\n> ```javascript\n> <js code text>\n> ```\n",
        ),
        LineBreak,
        LineBreak,
        Quote {
            indent: 3,
            content: "Use `\\` to escape special characters:\n```markdown\nLorem :::no-loc text=\"Find a \\\"Quotation\\\"\"::: Ipsum.\n```",
        },
        LineBreak,
        Heading {
            level: 2,
            text: "Selectors",
        },
        LineBreak,
        TextBlock {
            indent: 0,
            content: "Selectors are UI elements that let the user switch between multiple flavors of the same article. They are used in some docsets to address differences in implementation across technologies or platforms. Selectors are typically most applicable to our mobile platform content for developers.",
        },
        LineBreak,
        TextBlock {
            indent: 0,
            content: "Because the same selector Markdown goes in each article file that uses the selector, we recommend placing the selector for your article in an include file. Then you can reference that include file in all your article files that use the same selector.",
        },
        LineBreak,
        TextBlock {
            indent: 0,
            content: "There are two types of selectors: a single selector and a multi-selector.",
        },
        LineBreak,
        Heading {
            level: 3,
            text: "Single selector",
        },
        LineBreak,
        Code(
            MsMarkdownCode {
                content: "> [!div class=\"op_single_selector\"]\n> - [Universal Windows](../articles/notification-hubs-windows-store-dotnet-get-started/)\n> - [Windows Phone](../articles/notification-hubs-windows-phone-get-started/)\n> - [iOS](../articles/notification-hubs-ios-get-started/)\n> - [Android](../articles/notification-hubs-android-get-started/)\n> - [Kindle](../articles/notification-hubs-kindle-get-started/)\n> - [Baidu](../articles/notification-hubs-baidu-get-started/)\n> - [Xamarin.iOS](../articles/partner-xamarin-notification-hubs-ios-get-started/)\n> - [Xamarin.Android](../articles/partner-xamarin-notification-hubs-android-get-started/)",
                language: Markdown,
                source: None,
                range: None,
                id: None,
                highlight: None,
                interactive: None,
                indent: 0,
            },
        ),
        TextBlock {
            indent: 0,
            content: "... will be rendered like this:",
        },
        LineBreak,
        Heading {
            level: 3,
            text: "Multi-selector",
        },
        LineBreak,
        Code(
            MsMarkdownCode {
                content: "> [!div class=\"op_multi_selector\" title1=\"Platform\" title2=\"Backend\"]\n> - [(iOS | .NET)](./mobile-services-dotnet-backend-ios-get-started-push.md)\n> - [(iOS | JavaScript)](./mobile-services-javascript-backend-ios-get-started-push.md)\n> - [(Windows universal C# | .NET)](./mobile-services-dotnet-backend-windows-universal-dotnet-get-started-push.md)\n> - [(Windows universal C# | Javascript)](./mobile-services-javascript-backend-windows-universal-dotnet-get-started-push.md)\n> - [(Windows Phone | .NET)](./mobile-services-dotnet-backend-windows-phone-get-started-push.md)\n> - [(Windows Phone | Javascript)](./mobile-services-javascript-backend-windows-phone-get-started-push.md)\n> - [(Android | .NET)](./mobile-services-dotnet-backend-android-get-started-push.md)\n> - [(Android | Javascript)](./mobile-services-javascript-backend-android-get-started-push.md)\n> - [(Xamarin iOS | Javascript)](./partner-xamarin-mobile-services-ios-get-started-push.md)\n> - [(Xamarin Android | Javascript)](./partner-xamarin-mobile-services-android-get-started-push.md)",
                language: Markdown,
                source: None,
                range: None,
                id: None,
                highlight: None,
                interactive: None,
                indent: 0,
            },
        ),
        TextBlock {
            indent: 0,
            content: "... will be rendered like this:",
        },
        LineBreak,
        Heading {
            level: 2,
            text: "Subscript and superscript",
        },
        LineBreak,
        TextBlock {
            indent: 0,
            content: "You should only use subscript or superscript when necessary for technical accuracy, such as when writing about mathematical formulas. Don't use them for non-standard styles, such as footnotes.",
        },
        LineBreak,
        TextBlock {
            indent: 0,
            content: "For both subscript and superscript, use HTML:",
        },
        LineBreak,
        Code(
            MsMarkdownCode {
                content: "Hello <sub>This is subscript!</sub>",
                language: Html,
                source: None,
                range: None,
                id: None,
                highlight: None,
                interactive: None,
                indent: 0,
            },
        ),
        TextBlock {
            indent: 0,
            content: "This renders as follows:",
        },
        LineBreak,
        TextBlock {
            indent: 0,
            content: "Hello <sub>This is subscript!</sub>",
        },
        LineBreak,
        Code(
            MsMarkdownCode {
                content: "Goodbye <sup>This is superscript!</sup>",
                language: Html,
                source: None,
                range: None,
                id: None,
                highlight: None,
                interactive: None,
                indent: 0,
            },
        ),
        TextBlock {
            indent: 0,
            content: "This renders as follows:",
        },
        LineBreak,
        TextBlock {
            indent: 0,
            content: "Goodbye <sup>This is superscript!</sup>",
        },
        LineBreak,
        Heading {
            level: 2,
            text: "Tables",
        },
        LineBreak,
        TextBlock {
            indent: 0,
            content: "The simplest way to create a table in Markdown is to use pipes and lines. To create a standard table with a header, follow the first line with dashed line:",
        },
        LineBreak,
        Code(
            MsMarkdownCode {
                content: "|This is   |a simple   |table header|\n|----------|-----------|------------|\n|table     |data       |here        |\n|it doesn't|actually   |have to line up nicely!|",
                language: Markdown,
                source: None,
                range: None,
                id: None,
                highlight: None,
                interactive: None,
                indent: 0,
            },
        ),
        TextBlock {
            indent: 0,
            content: "This renders as follows:",
        },
        LineBreak,
        Table(
            Named(
                [
                    {
                        "This is": "table",
                        "a simple": "data",
                        "table header": "here",
                    },
                    {
                        "This is": "it doesn't",
                        "table header": "have to line up nicely!",
                        "a simple": "actually",
                    },
                ],
            ),
        ),
        TextBlock {
            indent: 0,
            content: "You can align the columns by using colons:",
        },
        LineBreak,
        Code(
            MsMarkdownCode {
                content: "| Fun                  | With                 | Tables          |\n| :------------------- | -------------------: |:---------------:|\n| left-aligned column  | right-aligned column | centered column |\n| $100                 | $100                 | $100            |\n| $10                  | $10                  | $10             |\n| $1                   | $1                   | $1              |",
                language: Markdown,
                source: None,
                range: None,
                id: None,
                highlight: None,
                interactive: None,
                indent: 0,
            },
        ),
        TextBlock {
            indent: 0,
            content: "Renders as follows:",
        },
        LineBreak,
        Table(
            Named(
                [
                    {
                        "Tables": "centered column",
                        "Fun": "left-aligned column",
                        "With": "right-aligned column",
                    },
                    {
                        "Tables": "$100",
                        "Fun": "$100",
                        "With": "$100",
                    },
                    {
                        "With": "$10",
                        "Fun": "$10",
                        "Tables": "$10",
                    },
                    {
                        "Fun": "$1",
                        "With": "$1",
                        "Tables": "$1",
                    },
                ],
            ),
        ),
        Alert {
            indent: 0,
            content: "The Learn Authoring Extension for VS Code makes it easy to add basic Markdown tables!\n\nYou can also use an [online table generator](http://www.tablesgenerator.com/markdown_tables).",
            alert_type: Tip,
        },
        Heading {
            level: 3,
            text: "Line breaks within words in any table cell",
        },
        LineBreak,
        TextBlock {
            indent: 0,
            content: "Long words in a Markdown table might make the table expand to the right navigation and become unreadable. You can solve that by allowing rendering to automatically insert line breaks within words when needed. Just wrap up the table with the custom class `[!div class=\"mx-tdBreakAll\"]`.",
        },
        LineBreak,
        TextBlock {
            indent: 0,
            content: "Here is a Markdown sample of a table with three rows that will be wrapped by a `div` with the class name `mx-tdBreakAll`.",
        },
        LineBreak,
        Code(
            MsMarkdownCode {
                content: "> [!div class=\"mx-tdBreakAll\"]\n> |Name|Syntax|Mandatory for silent installation?|Description|\n> |-------------|----------|---------|---------|\n> |Quiet|/quiet|Yes|Runs the installer, displaying no UI and no prompts.|\n> |NoRestart|/norestart|No|Suppresses any attempts to restart. By default, the UI will prompt before restart.|\n> |Help|/help|No|Provides help and quick reference. Displays the correct use of the setup command, including a list of all options and behaviors.|",
                language: Markdown,
                source: None,
                range: None,
                id: None,
                highlight: None,
                interactive: None,
                indent: 0,
            },
        ),
        TextBlock {
            indent: 0,
            content: "It will be rendered like this:",
        },
        LineBreak,
        Quote {
            indent: 0,
            content: "[!div class=\"mx-tdBreakAll\"]\n|Name|Syntax|Mandatory for silent installation?|Description|\n|-------------|----------|---------|---------|\n|Quiet|/quiet|Yes|Runs the installer, displaying no UI and no prompts.|\n|NoRestart|/norestart|No|Suppresses any attempts to restart. By default, the UI will prompt before restart.|\n|Help|/help|No|Provides help and quick reference. Displays the correct use of the setup command, including a list of all options and behaviors.|",
        },
        Heading {
            level: 3,
            text: "Line breaks within words in second column table cells",
        },
        LineBreak,
        TextBlock {
            indent: 0,
            content: "You might want line breaks to be automatically inserted within words only in the second column of a table. To limit the breaks to the second column, apply the class `mx-tdCol2BreakAll` by using the `div` wrapper syntax as shown earlier.",
        },
        LineBreak,
        Heading {
            level: 3,
            text: "Inconsistent column widths between tables",
        },
        LineBreak,
        TextBlock {
            indent: 0,
            content: "You may notice that the column widths of the tables in your articles look odd or inconsistent. This behavior occurs because the length of text within the cells determines the layout of the table. Unfortunately, there's no way to control how the tables render. This is a limitation of Markdown. Even though it would look nicer to have the width of table columns be consistent, this would have some disadvantages too:",
        },
        LineBreak,
        List(
            [
                MdListItem {
                    indent: 0,
                    list_type: Unordered,
                    content: "Interlacing HTML code with Markdown makes topics more complicated and discourages community contributions.",
                },
                MdListItem {
                    indent: 0,
                    list_type: Unordered,
                    content: "A table that you make look good for a specific screen size may end up looking unreadable at different screen sizes as it preempts responsive rendering.",
                },
            ],
        ),
        Heading {
            level: 3,
            text: "Data matrix tables",
        },
        LineBreak,
        TextBlock {
            indent: 0,
            content: "A data matrix table has both a header and a weighted first column, creating a matrix with an empty cell in the top left. Microsoft Learn has custom Markdown for data matrix tables:",
        },
        LineBreak,
        Code(
            MsMarkdownCode {
                content: "|                  |Header 1 |Header 2|\n|------------------|---------|--------|\n|**First column A**|Cell 1A  |Cell 2A |\n|**First column B**|Cell 1B  |Cell 2B |",
                language: Markdown,
                source: None,
                range: None,
                id: None,
                highlight: None,
                interactive: None,
                indent: 0,
            },
        ),
        TextBlock {
            indent: 0,
            content: "The example renders as:",
        },
        LineBreak,
        Table(
            Named(
                [
                    {
                        "Header 1": "Cell 1A",
                        "": "**First column A**",
                        "Header 2": "Cell 2A",
                    },
                    {
                        "": "**First column B**",
                        "Header 2": "Cell 2B",
                        "Header 1": "Cell 1B",
                    },
                ],
            ),
        ),
        TextBlock {
            indent: 0,
            content: "Every entry in the first column must be styled as bold (`**bold**`); otherwise the tables won't be accessible for screen readers or valid for Microsoft Learn.",
        },
        LineBreak,
        Alert {
            indent: 0,
            content: "The Learn Authoring Pack for VS Code includes a function to convert a regular Markdown table into a data matrix table. Just select the table, right-click, and select **Convert to data matrix table**.",
            alert_type: Tip,
        },
        Heading {
            level: 3,
            text: "HTML Tables",
        },
        LineBreak,
        TextBlock {
            indent: 0,
            content: "HTML tables aren't recommended for Microsoft Learn. They aren't human readable in the source - which is a key principle of Markdown.",
        },
        LineBreak,
        Metadata(
            MsMdMetadata {
                author: "mammerla",
                description: None,
                ms_author: "v-bbortree",
                ms_date: None,
                ms_service: None,
                ms_prod: Some(
                    "gaming",
                ),
                ms_topic: None,
                title: "Entity Documentation - minecraft:dweller",
                ms_custom: None,
                ms_reviewer: None,
                ms_subservice: None,
                technology: None,
                robots: None,
                no_loc: None,
            },
        ),
        LineBreak,
        Heading {
            level: 1,
            text: "Entity Documentation - minecraft:dweller",
        },
        LineBreak,
        TextBlock {
            indent: 0,
            content: "`minecraft:dweller` allows a mob to join and migrate between villages and other dwellings.",
        },
        LineBreak,
        Heading {
            level: 2,
            text: "Parameters",
        },
        LineBreak,
        Table(
            Named(
                [
                    {
                        "Default Value": "*not_set*",
                        "Name": "dwelling_type",
                        "Type": "String",
                        "Description": "The type of dwelling the mob wishes to join. Current Types: village",
                    },
                    {
                        "Type": "String",
                        "Name": "dweller_role",
                        "Description": "The role of which the mob plays in the dwelling. Current Roles: inhabitant, defender, hostile, passive.",
                        "Default Value": "*not set*",
                    },
                    {
                        "Description": "How often the mob checks on their dwelling status in ticks. Positive values only.",
                        "Default Value": "*not set*",
                        "Type": "Decimal",
                        "Name": "update_interval_base",
                    },
                    {
                        "Description": "The variant value in ticks that will be added to the update_interval_base.",
                        "Type": "Decimal",
                        "Default Value": "*not set*",
                        "Name": "update_interval_variant",
                    },
                    {
                        "Description": "Whether or not the mob can find and add POI's to the dwelling.",
                        "Name": "can_find_poi",
                        "Default Value": "*not set*",
                        "Type": "Boolean",
                    },
                    {
                        "Name": "first_founding_reward",
                        "Default Value": "*not set*",
                        "Type": "Integer",
                        "Description": "How much reputation should the players be rewarded on first founding?",
                    },
                    {
                        "Description": "Can this mob migrate between dwellings? Or does it only have its initial dwelling?",
                        "Name": "can_migrate",
                        "Default Value": "*not set*",
                        "Type": "Boolean",
                    },
                    {
                        "Type": "Float",
                        "Name": "dwelling_bounds_tolerance",
                        "Description": "A padding distance for checking if the mob is within the dwelling.",
                        "Default Value": "*not set*",
                    },
                    {
                        "Default Value": "*not set*",
                        "Description": "Allows the user to define a starting profession for this particular Dweller, instead of letting them choose organically. (They still need to gain experience from trading before this takes effect.)",
                        "Type": "String",
                        "Name": "preferred_profession",
                    },
                ],
            ),
        ),
        LineBreak,
        Heading {
            level: 2,
            text: "Example",
        },
        LineBreak,
        Code(
            MsMarkdownCode {
                content: "\"minecraft:dweller\": {\n    \"dwelling_type\": \"village\",\n    \"dweller_role\": \"inhabitant\",\n    \"preferred_profession\": \"toolsmith\",\n    \"update_interval_base\": 60,\n    \"update_interval_variant\": 40,\n    \"can_find_poi\": true,\n    \"can_migrate\": true,\n    \"first_founding_reward\": 5\n}",
                language: JSON,
                source: None,
                range: None,
                id: None,
                highlight: None,
                interactive: None,
                indent: 0,
            },
        ),
        Heading {
            level: 2,
            text: "Vanilla entities examples",
        },
        LineBreak,
        Heading {
            level: 3,
            text: "villager_v2",
        },
        LineBreak,
        Code(
            MsMarkdownCode {
                content: "\"minecraft:dweller\": {\n    \"dwelling_type\": \"  \",\n    \"dweller_role\": \"inhabitant\",\n    \"preferred_profession\": \"farmer\",\n    \"update_interval_base\": 60,\n    \"update_interval_variant\": 40,\n    \"can_find_poi\": true,\n    \"can_migrate\": true,\n    \"first_founding_reward\": 5\n}",
                language: JSON,
                source: None,
                range: None,
                id: None,
                highlight: None,
                interactive: None,
                indent: 0,
            },
        ),
        Heading {
            level: 2,
            text: "Vanilla entities using `minecraft:dweller`",
        },
        LineBreak,
        List(
            [
                MdListItem {
                    indent: 0,
                    list_type: Unordered,
                    content: "[cat](../../../../Source/VanillaBehaviorPack_Snippets/entities/cat.md)",
                },
                MdListItem {
                    indent: 0,
                    list_type: Unordered,
                    content: "[evocation_illager](../../../../Source/VanillaBehaviorPack_Snippets/entities/evocation_illager.md)",
                },
                MdListItem {
                    indent: 0,
                    list_type: Unordered,
                    content: "[iron_golem](../../../../Source/VanillaBehaviorPack_Snippets/entities/iron_golem.md)",
                },
                MdListItem {
                    indent: 0,
                    list_type: Unordered,
                    content: "[evocation_illager](../../../../Source/VanillaBehaviorPack_Snippets/entities/evocation_illager.md)",
                },
                MdListItem {
                    indent: 0,
                    list_type: Unordered,
                    content: "[pillager](../../../../Source/VanillaBehaviorPack_Snippets/entities/pillager.md)",
                },
                MdListItem {
                    indent: 0,
                    list_type: Unordered,
                    content: "[ravager](../../../../Source/VanillaBehaviorPack_Snippets/entities/ravager.md)",
                },
                MdListItem {
                    indent: 0,
                    list_type: Unordered,
                    content: "[villager_v2](../../../../Source/VanillaBehaviorPack_Snippets/entities/villager_v2.md)",
                },
                MdListItem {
                    indent: 0,
                    list_type: Unordered,
                    content: "[vindicator](../../../../Source/VanillaBehaviorPack_Snippets/entities/vindicator.md)",
                },
                MdListItem {
                    indent: 0,
                    list_type: Unordered,
                    content: "[witch](../../../../Source/VanillaBehaviorPack_Snippets/entities/witch.md)",
                },
            ],
        ),
        Metadata(
            MsMdMetadata {
                author: "kakinnun",
                description: Some(
                    "Update summary of Creator changes in Bedrock 1.19.70",
                ),
                ms_author: "kakinnun",
                ms_date: None,
                ms_service: None,
                ms_prod: Some(
                    "gaming",
                ),
                ms_topic: None,
                title: "1.19.70 Update Notes",
                ms_custom: None,
                ms_reviewer: None,
                ms_subservice: None,
                technology: None,
                robots: None,
                no_loc: None,
            },
        ),
        Heading {
            level: 1,
            text: "Minecraft Bedrock 1.19.70 Update Notes for Creators",
        },
        LineBreak,
        TextBlock {
            indent: 0,
            content: "Minecraft Bedrock has been updated to 1.19.70 and there are a number of changes of note for add-on creators.",
        },
        LineBreak,
        Heading {
            level: 2,
            text: "Holiday Creator Features",
        },
        LineBreak,
        TextBlock {
            indent: 0,
            content: "We continue to work on bringing the Holiday Creator Features out of experimental. Our current focus is on block components. The following block components are now available outside of the experimental toggle in 1.19.70.",
        },
        LineBreak,
        List(
            [
                MdListItem {
                    indent: 0,
                    list_type: Unordered,
                    content: "[Block Properties and Permutations](../Reference/Content/BlockReference/Examples/BlockPropertiesAndPermutations.md)",
                },
            ],
        ),
        Heading {
            level: 2,
            text: "Components",
        },
        LineBreak,
        TextBlock {
            indent: 0,
            content: "We've removed support for field \"data\" in the following commands:",
        },
        LineBreak,
        List(
            [
                MdListItem {
                    indent: 0,
                    list_type: Unordered,
                    content: "/clone",
                },
                MdListItem {
                    indent: 0,
                    list_type: Unordered,
                    content: "/execute",
                },
                MdListItem {
                    indent: 0,
                    list_type: Unordered,
                    content: "/fill",
                },
                MdListItem {
                    indent: 0,
                    list_type: Unordered,
                    content: "/setblock",
                },
                MdListItem {
                    indent: 0,
                    list_type: Unordered,
                    content: "/testforblock",
                },
            ],
        ),
        TextBlock {
            indent: 0,
            content: "**Example:**",
        },
        LineBreak,
        TextBlock {
            indent: 0,
            content: "No longer supported:",
        },
        LineBreak,
        TextBlock {
            indent: 0,
            content: "```/setblock ~ ~ ~ minecraft:wool 1```",
        },
        LineBreak,
        TextBlock {
            indent: 0,
            content: "Supported:",
        },
        LineBreak,
        TextBlock {
            indent: 0,
            content: "```/setblock ~ ~ ~ minecraft:wool [\"color\":\"orange\"]```",
        },
        LineBreak,
        Heading {
            level: 2,
            text: "Creator API",
        },
        LineBreak,
        TextBlock {
            indent: 0,
            content: "A reminder that a limited number of APIs were released out of experimental in [1.19.50](Update1.19.50.md). There are no new APIs leaving experimental in 1.19.70 but there are a number of new APIs behind experimental.",
        },
        LineBreak,
        TextBlock {
            indent: 0,
            content: "To use Beta APIs, you need to enable the Beta APIs experimental toggle.",
        },
        LineBreak,
        TextBlock {
            indent: 0,
            content: "**IMPORTANT BREAKING CHANGE:** The classes Location and BlockLocation no longer exist in the beta script API. All usages of these classes have been changed to use the Vector3 interface (that is, { x: 1, y: 2, z: 3} objects).",
        },
        LineBreak,
        TextBlock {
            indent: 0,
            content: "**BlockPermutation**",
        },
        LineBreak,
        TextBlock {
            indent: 0,
            content: "BlockPermutation has been significantly refactored! Every BlockPermutation now share a unique JavaScript handle so exact equality (===) will work for permutations that share exactly the same state values. We've also added utility methods that make interacting with permutations easier, which includes the removal of the XBlockProperty classes and now directly return properties (boolean | number | string) or a while collection of properties ( Record<string, boolean | number | string>).",
        },
        LineBreak,
        TextBlock {
            indent: 0,
            content: "Beta APIs will continue to be developed behind the Beta API experimental flag so if you want to use those, make sure you have that flag enabled and your manifest.json references will need to update to 1.1.0-beta. Read more about [script versioning](ScriptVersioning.md).",
        },
        LineBreak,
        TextBlock {
            indent: 0,
            content: "Looking ahead to upcoming releases, we are looking to release our next set of APIs out of experimental, potentially including read-only block and entity APIs.",
        },
        LineBreak,
        TextBlock {
            indent: 0,
            content: "Scripting is a powerful way to add complex behavior to your experience. It allows for a [professional development environment that includes profiling and hot reloading](./ScriptDeveloperTools.md).",
        },
        LineBreak,
        TextBlock {
            indent: 0,
            content: "[Get started with scripting](https://aka.ms/startwithmcscript).",
        },
        Metadata(
            MsMdMetadata {
                author: "mikehoffms",
                description: Some(
                    "Adding Xbox Live features to your game, such as Identity, Social features, Achievements, Cloud Storage, and Multiplayer features; and configuring Custom services to use Xbox Live.",
                ),
                ms_author: "v-mihof",
                ms_date: None,
                ms_service: None,
                ms_prod: None,
                ms_topic: Some(
                    "navigation",
                ),
                title: "Features",
                ms_custom: None,
                ms_reviewer: None,
                ms_subservice: None,
                technology: None,
                robots: None,
                no_loc: None,
            },
        ),
        LineBreak,
        Heading {
            level: 1,
            text: "Features",
        },
        LineBreak,
        Heading {
            level: 3,
            text: "In this section",
        },
        LineBreak,
        Table(
            Unnamed(
                [
                    [
                        "[Identity](identity/live-identity-nav.md)",
                        "User profile, authentication and sign-in, privileges, and privacy settings.",
                    ],
                    [
                        "[Player Data](player-data/live-playerdata-nav.md)",
                        "Achievements, player stats, leaderboards, and featured stats.",
                    ],
                    [
                        "[Social](social/live-social-nav.md)",
                        "Friends list (People system), official and in-game clubs, activity feed (presence strings), and reputation.",
                    ],
                    [
                        "[Multiplayer](multiplayer/live-multiplayer-nav.md)",
                        "Matchmaking, invites, voice and text chat, and networking.",
                    ],
                    [
                        "[Cloud Storage](cloud-storage/live-cloud-storage-nav.md)",
                        "Includes Connected Storage to store game state, and Title Storage to store player statistics and assets.",
                    ],
                    [
                        "[Custom services](custom-services/live-custom-services-nav.md)",
                        "Using Xbox Live with your own Web services; and configuring relying parties, single sign-on, and access policies.",
                    ],
                    [
                        "[General features](general/live-general-nav.md)",
                        "RTA (subscribes to state data, user statistics, and presence), TCUI (shows pre-defined user interface displays), and Xbox Live samples.",
                    ],
                ],
            ),
        ),
        LineBreak,
        Heading {
            level: 3,
            text: "See also",
        },
        LineBreak,
        List(
            [
                MdListItem {
                    indent: 0,
                    list_type: Unordered,
                    content: "[Features supported for each developer program](../get-started/join-dev-program/live-feature-comparison-table.md)",
                },
            ],
        ),
        Metadata(
            MsMdMetadata {
                author: "mammerla",
                description: Some(
                    "A guide covering many of the most common commands that are used within Minecraft: Bedrock Edition",
                ),
                ms_author: "v-jillheaden",
                ms_date: None,
                ms_service: None,
                ms_prod: Some(
                    "gaming",
                ),
                ms_topic: None,
                title: "Popular Commands",
                ms_custom: None,
                ms_reviewer: None,
                ms_subservice: None,
                technology: None,
                robots: None,
                no_loc: None,
            },
        ),
        LineBreak,
        Heading {
            level: 1,
            text: "Popular Commands",
        },
        LineBreak,
        TextBlock {
            indent: 0,
            content: "There are a number of commands that are vital to many command systems. This list will go over the more popular and useful commands that are likely to be used in some fashion.",
        },
        LineBreak,
        TextBlock {
            indent: 0,
            content: "In this tutorial you will learn the following:",
        },
        LineBreak,
        Heading {
            level: 3,
            text: "Requirements",
        },
        LineBreak,
        TextBlock {
            indent: 0,
            content: "It's recommended that the following be completed before beginning this tutorial.",
        },
        LineBreak,
        List(
            [
                MdListItem {
                    indent: 0,
                    list_type: Unordered,
                    content: "[Introduction to Commands](CommandsIntroduction.md)",
                },
                MdListItem {
                    indent: 0,
                    list_type: Unordered,
                    content: "[Getting Started with Command Blocks](CommandBlocks.md)",
                },
            ],
        ),
        Heading {
            level: 2,
            text: "/help",
        },
        LineBreak,
        TextBlock {
            indent: 0,
            content: "First and foremost is the `/help` command.",
        },
        LineBreak,
        Code(
            MsMarkdownCode {
                content: "/help <page: int>",
                language: None,
                source: None,
                range: None,
                id: None,
                highlight: None,
                interactive: None,
                indent: 0,
            },
        ),
        TextBlock {
            indent: 0,
            content: "Running `/help` in the chat by itself will list every command you can run. However, since there are so many, they have been divided up into several pages, where you are shown only one page at a time. Supplying a page number will change which page of commands to show.",
        },
        LineBreak,
        TextBlock {
            indent: 0,
            content: "Whenever you want to know the usage and all of the syntaxes of a command, you can use `/help` followed by the name of the command. This can be helpful if you want an overview of the command.",
        },
        LineBreak,
        Code(
            MsMarkdownCode {
                content: "/help [command: CommandName]",
                language: None,
                source: None,
                range: None,
                id: None,
                highlight: None,
                interactive: None,
                indent: 0,
            },
        ),
        Heading {
            level: 2,
            text: "/effect",
        },
        LineBreak,
        TextBlock {
            indent: 0,
            content: "Adding and removing status effects is done with the `/effect` command. All status effects on the target can also be cleared.",
        },
        LineBreak,
        Heading {
            level: 2,
            text: "/execute",
        },
        LineBreak,
        TextBlock {
            indent: 0,
            content: "The primary function of the `/execute` command is to modify the executor and execution origin of a nested command. However, its syntax allows for some conditional command execution that would otherwise be missing in functions.",
        },
        LineBreak,
        Code(
            MsMarkdownCode {
                content: "/execute <origin: target> <position: x y z> <command: command>\n\n/execute <origin: target> <position: x y z> detect <detectPos: x y z> <block: Block> <data: int> <command: command>",
                language: None,
                source: None,
                range: None,
                id: None,
                highlight: None,
                interactive: None,
                indent: 0,
            },
        ),
        TextBlock {
            indent: 0,
            content: "For example, to detect if there's a specific block beneath the player and run a command as a result, the \"detect\" argument can be used instead of having a nested `/testforblock` command.",
        },
        LineBreak,
        Code(
            MsMarkdownCode {
                content: "/execute @a ~ ~ ~ detect ~ ~-1 ~ grass 0 say Player is standing on top of grass.",
                language: None,
                source: None,
                range: None,
                id: None,
                highlight: None,
                interactive: None,
                indent: 0,
            },
        ),
        Heading {
            level: 2,
            text: "/gamemode",
        },
        LineBreak,
        TextBlock {
            indent: 0,
            content: "When developing content, most often you'll be playing in creative mode. However, testing your content may require you to enter survival or adventure mode. The `/gamemode` command allows you to change your current game mode.",
        },
        LineBreak,
        Code(
            MsMarkdownCode {
                content: "/gamemode <gameMode: GameMode> [player: target]\n/gamemode <gameMode: int> [player: target]",
                language: None,
                source: None,
                range: None,
                id: None,
                highlight: None,
                interactive: None,
                indent: 0,
            },
        ),
        TextBlock {
            indent: 0,
            content: "You have several options for the game mode: the full name, a single character, or a numeric representation. The accepted values are \"survival\" (\"s\" or 0), \"creative\" (\"c\" or 1), and \"adventure\" (\"a\" or 2). There is also \"default\" (\"d\"), which sets your game mode to whatever the world's default game mode is. The following will set your own game mode to creative when you run it in the chat.",
        },
        LineBreak,
        Code(
            MsMarkdownCode {
                content: "/gamemode creative",
                language: None,
                source: None,
                range: None,
                id: None,
                highlight: None,
                interactive: None,
                indent: 0,
            },
        ),
        TextBlock {
            indent: 0,
            content: "You can also change the game mode of a specific player using target selectors, which can be helpful when controlling gameplay aspects (such as the inability to mine anything in adventure mode). The following changes the game mode of all players with the \"sometag\" tag to adventure mode.",
        },
        LineBreak,
        Code(
            MsMarkdownCode {
                content: "/gamemode adventure @a[tag=sometag]",
                language: None,
                source: None,
                range: None,
                id: None,
                highlight: None,
                interactive: None,
                indent: 0,
            },
        ),
        Heading {
            level: 2,
            text: "/gamerule",
        },
        LineBreak,
        TextBlock {
            indent: 0,
            content: "This `/gamerule` command changes specific gameplay aspects and has options helpful for development. There are a large number of gamerules, which can be listed using the chat's auto-complete feature.",
        },
        LineBreak,
        Code(
            MsMarkdownCode {
                content: "/gamerule <rule: BoolGameRule> [value: Boolean]\n/gamerule <rule: IntGameRule> [value: int]",
                language: None,
                source: None,
                range: None,
                id: None,
                highlight: None,
                interactive: None,
                indent: 0,
            },
        ),
        TextBlock {
            indent: 0,
            content: "Some gamerules accept only true or false as the value of the rule (\"BoolGameRule\") while others only accept integers (\"IntGameRule\"). The primary gamerules crucial for development, which are all booleans, are \"commandblockoutput\", \"sendcommandfeedback\", and \"commandblocksenabled\", \"dodaylightcycle\", and \"doweathercycle\" listed below.",
        },
        LineBreak,
        Heading {
            level: 3,
            text: "\"commandblockoutput\"",
        },
        LineBreak,
        TextBlock {
            indent: 0,
            content: "You may have noticed that all of the previous command examples send an output to the chat. This can be a nuisance, especially for commands running every tick. The \"commandblockoutput\" gamerule, when set to false, will disable that chat output.",
        },
        LineBreak,
        Code(
            MsMarkdownCode {
                content: "/gamerule commandblockoutput false",
                language: None,
                source: None,
                range: None,
                id: None,
                highlight: None,
                interactive: None,
                indent: 0,
            },
        ),
        Heading {
            level: 3,
            text: "\"sendcommandfeedback\"",
        },
        LineBreak,
        TextBlock {
            indent: 0,
            content: "Disabling command block output does not disable all feedback. There are some commands, such as `/xp`, that will continue providing feedback even when command block output is disabled. The \"sendcommandfeedback\" gamerule will disable these messages as well, allowing for a completely silent command system.",
        },
        LineBreak,
        Code(
            MsMarkdownCode {
                content: "/gamerule sendcommandfeedback false",
                language: None,
                source: None,
                range: None,
                id: None,
                highlight: None,
                interactive: None,
                indent: 0,
            },
        ),
        Heading {
            level: 3,
            text: "\"commandblocksenabled\"",
        },
        LineBreak,
        TextBlock {
            indent: 0,
            content: "To quickly enable or disable command blocks in the world, the \"commandblocksenabled\" can be toggled on and off. This can be especially helpful if a repeating command block is repeatedly teleporting you.",
        },
        LineBreak,
        Code(
            MsMarkdownCode {
                content: "/gamerule commandblocksenabled false",
                language: None,
                source: None,
                range: None,
                id: None,
                highlight: None,
                interactive: None,
                indent: 0,
            },
        ),
        Heading {
            level: 3,
            text: "\"dodaylightcycle\"",
        },
        LineBreak,
        TextBlock {
            indent: 0,
            content: "Either for development or for gameplay concerns, you can choose to prevent the daylight cycle from moving. This means that whatever time you set the game to (such as with the [`/time` command](#time-set)), it will stay at that time.",
        },
        LineBreak,
        Code(
            MsMarkdownCode {
                content: "/gamerule dodaylightcycle false",
                language: None,
                source: None,
                range: None,
                id: None,
                highlight: None,
                interactive: None,
                indent: 0,
            },
        ),
        Heading {
            level: 3,
            text: "\"doweathercycle\"",
        },
        LineBreak,
        TextBlock {
            indent: 0,
            content: "As with \"dodaylightcycle\", you may also want to control the weather cycle. If you intend to have a happy setting, a thunderstorm setting in may not achieve the best effect. When \"doweathercycle\" is disabled, the weather will stay as it is, including when set with the [`/weather` command](#weather).",
        },
        LineBreak,
        Code(
            MsMarkdownCode {
                content: "/gamerule doweathercycle false",
                language: None,
                source: None,
                range: None,
                id: None,
                highlight: None,
                interactive: None,
                indent: 0,
            },
        ),
        Heading {
            level: 2,
            text: "/give, /clear, /replaceitem",
        },
        LineBreak,
        TextBlock {
            indent: 0,
            content: "This trio of commands manipulate the player's inventory, though `/replaceitem` can also modify non-player entity inventories and blocks with inventories. The `/give` command can provide items, the `/clear` command can remove items, and the `/replaceitem` command can place items in specific slots in the inventory.",
        },
        LineBreak,
        Heading {
            level: 2,
            text: "/locate",
        },
        LineBreak,
        TextBlock {
            indent: 0,
            content: "Finds the nearest specified biome or structure if it exists in the current dimension. Prints an error if it does not.",
        },
        LineBreak,
        Code(
            MsMarkdownCode {
                content: "/locate biome <biome name>\n/locate structure <structure name>",
                language: None,
                source: None,
                range: None,
                id: None,
                highlight: None,
                interactive: None,
                indent: 0,
            },
        ),
        TextBlock {
            indent: 0,
            content: "The locate command takes two arguments: the first argument specifies whether to locate a biome or a structure, the second argument specifies the name of the biome or structure to be located.",
        },
        LineBreak,
        Code(
            MsMarkdownCode {
                content: "/locate biome beach\n/locate structure village",
                language: None,
                source: None,
                range: None,
                id: None,
                highlight: None,
                interactive: None,
                indent: 0,
            },
        ),
        Heading {
            level: 2,
            text: "/scoreboard",
        },
        LineBreak,
        TextBlock {
            indent: 0,
            content: "The `/scoreboard` command is a powerful method of keeping track of numeric values on a per-entity basis, as well as performing mathematical operations with commands. The first step is creating an objective and (optionally) displaying it on the sidebar.",
        },
        LineBreak,
        Code(
            MsMarkdownCode {
                content: "/scoreboard objectives add objectiveA dummy\n\n/scoreboard objectives setdisplay sidebar objectiveA",
                language: None,
                source: None,
                range: None,
                id: None,
                highlight: None,
                interactive: None,
                indent: 0,
            },
        ),
        TextBlock {
            indent: 0,
            content: "The simplest course of action would be rewarding the player with a point if they accomplish some task.",
        },
        LineBreak,
        Code(
            MsMarkdownCode {
                content: "/scoreboard players add @p objectiveA 1",
                language: None,
                source: None,
                range: None,
                id: None,
                highlight: None,
                interactive: None,
                indent: 0,
            },
        ),
        TextBlock {
            indent: 0,
            content: "Afterwards, target selectors can be used to select players who achieve a certain number of points.",
        },
        LineBreak,
        Code(
            MsMarkdownCode {
                content: "@a[scores={objectiveA=10..}]",
                language: None,
                source: None,
                range: None,
                id: None,
                highlight: None,
                interactive: None,
                indent: 0,
            },
        ),
        Heading {
            level: 2,
            text: "/setblock, /fill, /clone",
        },
        LineBreak,
        TextBlock {
            indent: 0,
            content: "These commands change the physical blocks in the world. The `/setblock` command can set a single block while the `/fill` command can set multiple of the same block. The `/clone` command, on the other hand, will take a copy of blocks from one area and paste it into another.",
        },
        LineBreak,
        Heading {
            level: 2,
            text: "/summon",
        },
        LineBreak,
        TextBlock {
            indent: 0,
            content: "The `/summon` command is used to spawn a new entity into the world, from cows to sheep to your own custom entities.",
        },
        LineBreak,
        Heading {
            level: 2,
            text: "/setworldspawn",
        },
        LineBreak,
        TextBlock {
            indent: 0,
            content: "If you have a specific location that you want players new to the world or players who die to spawn at, the `/setworldspawn` command provides that ability.",
        },
        LineBreak,
        Code(
            MsMarkdownCode {
                content: "/setworldspawn [spawnPoint: x y z]",
                language: None,
                source: None,
                range: None,
                id: None,
                highlight: None,
                interactive: None,
                indent: 0,
            },
        ),
        TextBlock {
            indent: 0,
            content: "Note that players who die after they have set their spawn with a bed will still respawn at their bed.",
        },
        LineBreak,
        Heading {
            level: 2,
            text: "/tag",
        },
        LineBreak,
        TextBlock {
            indent: 0,
            content: "Similar to `/scoreboard`, the `/tag` command allows you to keep track of string values on a per-entity basis. Tags would be used when a numeric value is not needed, such as for \"true or false\" situations. For example, you could tag entities as being a boss and later target those same entities based on that tag.",
        },
        LineBreak,
        Code(
            MsMarkdownCode {
                content: "/tag @e[type=sheep] add boss\n/tag @e[type=minecart] add boss",
                language: None,
                source: None,
                range: None,
                id: None,
                highlight: None,
                interactive: None,
                indent: 0,
            },
        ),
        Code(
            MsMarkdownCode {
                content: "/say Bosses: @e[tag=boss]",
                language: None,
                source: None,
                range: None,
                id: None,
                highlight: None,
                interactive: None,
                indent: 0,
            },
        ),
        Heading {
            level: 2,
            text: "/tellraw, /titleraw",
        },
        LineBreak,
        TextBlock {
            indent: 0,
            content: "The `/say`, `/tell`, and `/title` commands are not ideal when presenting information as they are not open to translation. The `/tellraw` and `/titleraw` commands can be translated using a JSON input for the message. The `/tellraw` command is also clearer in intent as the message is not accompanied with a \"whisper\" statement.",
        },
        LineBreak,
        Code(
            MsMarkdownCode {
                content: "/tellraw @a {\"rawtext\":[{\"translate\":\"commands.testfor.success\",\"with\":[\"PlayerName\"]}]}\n\n/tellraw @a {\"rawtext\":[{\"text\":\"Hello World\"}]}",
                language: None,
                source: None,
                range: None,
                id: None,
                highlight: None,
                interactive: None,
                indent: 0,
            },
        ),
        Heading {
            level: 2,
            text: "/testfor, /testforblock, /testforblocks",
        },
        LineBreak,
        TextBlock {
            indent: 0,
            content: "These commands test for the existence of an entity, block, and a copy of a block structure. While these commands can be useful alongside conditional command blocks, they are less useful in functions as there is no equivalent conditional setting in functions.",
        },
        LineBreak,
        TextBlock {
            indent: 0,
            content: "Both the `/testfor` and `/testforblock` commands can generally be skipped over in favor of `/execute`, which supports running a command based on the existence of an entity and running a command based on the existence of a block.",
        },
        LineBreak,
        Heading {
            level: 2,
            text: "/time set",
        },
        LineBreak,
        TextBlock {
            indent: 0,
            content: "Changing the gameplay environment can be essential for providing the right atmosphere. Changing the time of day is one method of doing so.",
        },
        LineBreak,
        Code(
            MsMarkdownCode {
                content: "/time set <amount: int>\n/time set <time: TimeSpec>",
                language: None,
                source: None,
                range: None,
                id: None,
                highlight: None,
                interactive: None,
                indent: 0,
            },
        ),
        TextBlock {
            indent: 0,
            content: "You can either provide an integer which represents a precise time of the day, or \"TimeSpec\" can be set to one of the following values to more easily select common times of day: \"day\", \"midnight\", \"night\", \"noon\", \"sunrise\", and \"sunset\". For example, quickly setting the sun directly above the player would use the following:",
        },
        LineBreak,
        Code(
            MsMarkdownCode {
                content: "/time set noon",
                language: None,
                source: None,
                range: None,
                id: None,
                highlight: None,
                interactive: None,
                indent: 0,
            },
        ),
        Heading {
            level: 2,
            text: "/tp or /teleport",
        },
        LineBreak,
        TextBlock {
            indent: 0,
            content: "Sometimes you may want to provide an easy means of transportation or need a form of controlling where the player (or even non-player entity) is. The `/tp` command will teleport targeted players to specific locations, or even to other entities. The syntaxes for `/tp` may look daunting at first, but note many branch to and from the same options.",
        },
        LineBreak,
        Code(
            MsMarkdownCode {
                content: "/tp <victim: target> <destination: target> [checkForBlocks: Boolean]\n/tp <victim: target> <destination: x y z> [checkForBlocks: Boolean]\n/tp <victim: target> <destination: x y z> [yRot: value] [xRot: value] [checkForBlocks: Boolean]\n/tp <victim: target> <destination: x y z> facing <lookAtEntity: target> [checkForBlocks: Boolean]\n/tp <victim: target> <destination: x y z> facing <lookAtPosition: x y z> [checkForBlocks: Boolean]",
                language: None,
                source: None,
                range: None,
                id: None,
                highlight: None,
                interactive: None,
                indent: 0,
            },
        ),
        TextBlock {
            indent: 0,
            content: "At its simplest, you can teleport the player to a specific coordinate location.",
        },
        LineBreak,
        Code(
            MsMarkdownCode {
                content: "/tp @p 100 50 100",
                language: None,
                source: None,
                range: None,
                id: None,
                highlight: None,
                interactive: None,
                indent: 0,
            },
        ),
        TextBlock {
            indent: 0,
            content: "You can also teleport them to another player or entity. The destination target must resolve to a single entity, so you have to use a target selector that can only target one entity or ensure there are no duplicates of the intended target (such as using the [`/tag` command](#tag) to identify targets).",
        },
        LineBreak,
        Code(
            MsMarkdownCode {
                content: "/tp Steve Alex\n/tp @p @e[type=minecraft:armor_stand,c=1]\n/tp @a @e[type=minecraft:creeper,tag=destination_1]",
                language: None,
                source: None,
                range: None,
                id: None,
                highlight: None,
                interactive: None,
                indent: 0,
            },
        ),
        TextBlock {
            indent: 0,
            content: "The common option \"checkForBlocks\" defaults to false, but when it is set to true, it will prevent teleporting the player if the location they would end up in is occupied by blocks.",
        },
        LineBreak,
        TextBlock {
            indent: 0,
            content: "This command teleports Steve to coordinates [50, 63, 50] and makes sure there are no blocks in the way.",
        },
        LineBreak,
        Code(
            MsMarkdownCode {
                content: "/tp Steve 50 63 50 true",
                language: None,
                source: None,
                range: None,
                id: None,
                highlight: None,
                interactive: None,
                indent: 0,
            },
        ),
        Alert {
            indent: 0,
            content: "One of the differences between using commands and commands blocks is that `/tp @s ...` will not work in a command block.",
            alert_type: Note,
        },
        Heading {
            level: 2,
            text: "/weather",
        },
        LineBreak,
        TextBlock {
            indent: 0,
            content: "Like [`/time set`](#time-set), the `/weather` command can be used to change the environment. If you want a specific type of weather to occur, this would be the command you would use.",
        },
        LineBreak,
        Code(
            MsMarkdownCode {
                content: "/weather <clear|rain|thunder> [duration: int]",
                language: None,
                source: None,
                range: None,
                id: None,
                highlight: None,
                interactive: None,
                indent: 0,
            },
        ),
        TextBlock {
            indent: 0,
            content: "The optional duration is the number of game ticks that the selected weather will last for. 20 game ticks is one second, so for each second you want the weather to last, multiply it by twenty. The following sets the weather to thunder for 30 seconds, which is 600 ticks.",
        },
        LineBreak,
        Code(
            MsMarkdownCode {
                content: "/weather thunder 600",
                language: None,
                source: None,
                range: None,
                id: None,
                highlight: None,
                interactive: None,
                indent: 0,
            },
        ),
        TextBlock {
            indent: 0,
            content: "Be sure that the [\"doweathercycle\" gamerule](#doweathercycle) is false if you intend on making use of the duration. If it is true, the weather cycle will not occur, rendering the duration useless.",
        },
        LineBreak,
        Heading {
            level: 2,
            text: "What's Next?",
        },
        LineBreak,
        TextBlock {
            indent: 0,
            content: "After learning about commands and target selectors, command blocks, and a variety of useful commands, it's time to put them together and create a small project in the form of a Complete the Monument system.",
        },
        LineBreak,
        Metadata(
            MsMdMetadata {
                author: "mammerla",
                description: None,
                ms_author: "mikeam",
                ms_date: None,
                ms_service: None,
                ms_prod: Some(
                    "gaming",
                ),
                ms_topic: None,
                title: "Recipe Documentation - Potion Brewing Container Recipe",
                ms_custom: None,
                ms_reviewer: None,
                ms_subservice: None,
                technology: None,
                robots: None,
                no_loc: None,
            },
        ),
        LineBreak,
        Heading {
            level: 1,
            text: "Recipe Documentation - Potion Brewing Container Recipe",
        },
        LineBreak,
        TextBlock {
            indent: 0,
            content: "Represents a Potion Brewing Container Recipe.",
        },
        LineBreak,
        Heading {
            level: 2,
            text: "Parameters",
        },
        LineBreak,
        Table(
            Named(
                [
                    {
                        "Description": "Input potion used in the brewing container recipe.",
                        "Name": "input",
                        "Type": "potion",
                    },
                    {
                        "Name": "output",
                        "Description": "Output potion from the brewing container recipe.",
                        "Type": "potion",
                    },
                    {
                        "Name": "reagent",
                        "Type": "item",
                        "Description": "Item used in the brewing container recipe with the input potion.",
                    },
                    {
                        "Description": "Item that can create the Brewing Container Recipe, such as \"brewing_stand\".",
                        "Name": "tags",
                        "Type": "String array",
                    },
                ],
            ),
        ),
        Heading {
            level: 2,
            text: "Potion Brewing Container Recipe Example",
        },
        LineBreak,
        Code(
            MsMarkdownCode {
                content: "{\n\"format_version\": \"1.17\",\n    \"minecraft:recipe_brewing_container\": {\n    \"description\": {\n        \"identifier\": \"minecraft:brew_potion_sulphur\"\n    },\n    \"tags\": [ \"brewing_stand\" ],\n    \"input\": \"minecraft:potion\",\n    \"reagent\": \"minecraft:gunpowder\",\n    \"output\": \"minecraft:splash_potion\"\n    }\n}",
                language: JSON,
                source: None,
                range: None,
                id: None,
                highlight: None,
                interactive: None,
                indent: 0,
            },
        ),
        Heading {
            level: 2,
            text: "Vanilla Example",
        },
        LineBreak,
        Heading {
            level: 3,
            text: "Brew Potion Sulphur",
        },
        LineBreak,
        Code(
            MsMarkdownCode {
                content: "",
                language: JSON,
                source: Some(
                    "../../../../Source/VanillaBehaviorPack/recipes/brew_potion_sulphur.json",
                ),
                range: None,
                id: None,
                highlight: None,
                interactive: None,
                indent: 0,
            },
        ),
    ],
}