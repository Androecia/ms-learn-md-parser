---
title: Markdown reference for Microsoft Learn
description: Learn the Markdown features and syntax used in Microsoft Learn content.
author: meganbradley
ms.author: mbradley
ms.date: 11/09/2021
ms.topic: contributor-guide
ms.prod: non-product-specific
ms.custom: external-contributor-guide
---

# Learn Markdown reference

This article provides an alphabetical reference for writing Markdown for [Microsoft Learn](/).

[Markdown](https://daringfireball.net/projects/markdown/) is a lightweight markup language with plain text formatting syntax. The Microsoft Learn platform supports [CommonMark](https://commonmark.org/) compliant Markdown parsed through the [Markdig](https://github.com/lunet-io/markdig) parsing engine. Microsoft Learn also supports custom Markdown extensions that provide richer content on the Microsoft Learn site.

You can use any text editor to write Markdown, but we recommend [Visual Studio Code](https://code.visualstudio.com/) with the [Learn Authoring Pack](https://aka.ms/DocsAuthoringPack). The Learn Authoring Pack provides editing tools and preview functionality that lets you see what your articles will look like when rendered on Microsoft Learn.

## Alerts (Note, Tip, Important, Caution, Warning)

Alerts are a Markdown extension to create block quotes that render on Microsoft Learn with colors and icons that indicate the significance of the content.

Avoid notes, tips, and important boxes. Readers tend to skip over them. It's better to put that info directly into the article text.

If you need to use alerts, limit them to one or two per article. Multiple notes should never be next to each other in an article.

The following alert types are supported:

```md
> [!NOTE]
> Information the user should notice even if skimming.

> [!TIP]
> Optional information to help a user be more successful.

> [!IMPORTANT]
> Essential information required for user success.

> [!CAUTION]
> Negative potential consequences of an action.

> [!WARNING]
> Dangerous certain consequences of an action.
```

These alerts look like this on Microsoft Learn:

> [!NOTE]
> Information the user should notice even if skimming.

> [!TIP]
> Optional information to help a user be more successful.

> [!IMPORTANT]
> Essential information required for user success.

> [!CAUTION]
> Negative potential consequences of an action.

> [!WARNING]
> Dangerous certain consequences of an action.

## Angle brackets

If you use angle brackets in text in your file (for example, to denote a placeholder), you need to manually encode the angle brackets. Otherwise, Markdown thinks that they're intended to be an HTML tag.

For example, encode `<script name>` as `&lt;script name&gt;` or `\<script name>`.

Angle brackets don't have to be escaped in text formatted as inline code or in code blocks.

## Apostrophes and quotation marks

If you copy from Word into a Markdown editor, the text might contain "smart" (curly) apostrophes or quotation marks. These need to be encoded or changed to basic apostrophes or quotation marks. Otherwise, you end up with things like this when the file is published: Itâ&euro;&trade;s

Here are the encodings for the "smart" versions of these punctuation marks:

- Left (opening) quotation mark: `&#8220;`
- Right (closing) quotation mark: `&#8221;`
- Right (closing) single quotation mark or apostrophe: `&#8217;`
- Left (opening) single quotation mark (rarely used): `&#8216;`

> [!TIP]
> To avoid "smart" characters in your Markdown files, rely on the Learn Authoring Pack's smart quote replacement feature. For more information, see [smart quote replacement](docs-authoring/smart-quote-replacement.md).

## Blockquotes

Blockquotes are created using the `>` character:

```md
> This is a blockquote. It is usually rendered indented and with a different background color.
```

The preceding example renders as follows:

> This is a blockquote. It is usually rendered indented and with a different background color.

## Bold and italic text

To format text as **bold**, enclose it in two asterisks:

```md
This text is **bold**.
```

To format text as *italic*, enclose it in a single asterisk:

```md
This text is *italic*.
```

To format text as both ***bold and italic***, enclose it in three asterisks:

```md
This text is both ***bold and italic***.
```

For guidance on when to use bold and italic text, see [text formatting guidelines](text-formatting-guidelines.md).

## Code snippets

Learn Markdown supports the placement of code snippets both inline in a sentence and as a separate "fenced" block between sentences. For more information, see [How to add code to docs](code-in-docs.md).

## Columns

The **columns** Markdown extension gives authors the ability to add column-based content layouts that are more flexible and powerful than basic Markdown tables, which are only suited for true tabular data. You can add up to four columns, and use the optional `span` attribute to merge two or more columns.

While the **columns** extension still works, we no longer recommend it for creating custom layouts. We've found that many custom column layouts have accessibility issues or otherwise violate the style guidelines. Don't create custom layouts. Use standard Microsoft Learn features.

The syntax for columns is as follows:

```md
:::row:::
   :::column span="":::
      Content...
   :::column-end:::
   :::column span="":::
      More content...
   :::column-end:::
:::row-end:::
```

Columns should only contain basic Markdown, including images. Headings, tables, tabs, and other complex structures shouldn't be included. A row can't have any content outside of column.

For example, the following Markdown creates one column that spans two column widths, and one standard (no `span`) column:

```md
:::row:::
   :::column span="2":::
      **This is a 2-span column with lots of text.**

      Lorem ipsum dolor sit amet, consectetur adipiscing elit. Donec vestibulum mollis nunc
      ornare commodo. Nullam ac metus imperdiet, rutrum justo vel, vulputate leo. Donec
      rutrum non eros eget consectetur.
   :::column-end:::
   :::column span="":::
      **This is a single-span column with an image in it.**

      ![Doc.U.Ment](media/markdown-reference/document.png)
   :::column-end:::
:::row-end:::
```

This renders as follows:

:::row:::
   :::column span="2":::
      **This is a 2-span column with lots of text.**

      Lorem ipsum dolor sit amet, consectetur adipiscing elit. Donec vestibulum mollis nunc
      ornare commodo. Nullam ac metus imperdiet, rutrum justo vel, vulputate leo. Donec
      rutrum non eros eget consectetur.
   :::column-end:::
   :::column span="":::
      **This is a single-span column with an image in it.**

      ![Doc.U.Ment](media/markdown-reference/document.png)
   :::column-end:::
:::row-end:::

## Comments

Microsoft Learn supports HTML comments if you must comment out sections of your article:

```html
<!--- Here's my comment --->
```

> [!WARNING]
> Do not put private or sensitive information in HTML comments. Microsoft Learn carries HTML comments through to the published HTML that goes public. While HTML comments are invisible to the reader's eye, they are exposed in the HTML underneath.

## Headings

Microsoft Learn supports six levels of Markdown headings:

```md
# This is a first level heading (H1)

## This is a second level heading (H2)

...

###### This is a sixth level heading (H6)
```

- There must be a space between the last `#` and heading text.
- Each Markdown file must have one and only one H1 heading.
- The H1 heading must be the first content in the file after the YML metadata block.
- H2 headings automatically appear in the right-hand navigating menu of the published file. Lower-level headings don't appear, so use H2s strategically to help readers navigate your content.
- HTML headings, such as `<h1>`, aren't recommended, and in some cases will cause build warnings.
- You can link to individual headings in a file via [bookmark links](how-to-write-links.md#explicit-anchor-links).

## HTML

Although Markdown supports inline HTML, HTML isn't recommended for publishing to Microsoft Learn, and except for a limited list of values will cause build errors or warnings.

## Images

The following file types are supported by default for images:

- .jpg
- .png

To support other image types, such as .gif, you must add them as resources in *docfx.json*:

```md
"resource": [
  {
    "files" : [
      "**/*.png",
      "**/*.jpg,
      "**/*.gif"
    ],
```

### Standard conceptual images (default Markdown)

The basic Markdown syntax to embed an image is:

```Markdown
![<alt text>](<folderPath>)

Example:
![alt text for image](../images/Introduction.png)
```
![alt text for image](../images/Introduction.png)
Where `<alt text>` is a brief description of the image and `<folder path>` is a relative path to the image. Alternate text is required for screen readers for the visually impaired. It's also useful if there's a site bug where the image can't render.

Underscores in alt text aren't rendered properly unless you escape them by prefixing them with a backslash (`\_`). However, don't copy file names for use as alt text. For example, instead of this:

```md
![ADextension_2FA_Configure_Step4](./media/bogusfilename/ADextension_2FA_Configure_Step4.PNG)
```

Write this:

```md
![Active Directory extension for two-factor authentication, step 4: Configure](./media/bogusfilename/ADextension_2FA_Configure_Step4.PNG)
```

### Standard conceptual images (Learn Markdown)

The custom `:::image:::` extension on Microsoft Learn supports standard images, complex images, and icons.

For standard images, the older Markdown syntax will still work, but the new extension is recommended because it supports more powerful functionality, such as specifying a localization scope that's different from the parent topic. Other advanced functionality, such as selecting from the shared image gallery instead of specifying a local image, will be available in the future. The new syntax is as follows:

```md
:::image type="content" source="<folderPath>" alt-text="<alt text>":::
```
:::image type="content" source="<folderPath>" alt-text="<alt text>":::

If `type="content"` (the default), both `source` and `alt-text` are required.

### Complex images with long descriptions

You can also use this extension to add an image with a long description that is read by screen readers but not rendered visually on the published page. Long descriptions are an accessibility requirement for complex images, such as graphs. The syntax is the following:

```md
:::image type="complex" source="<folderPath>" alt-text="<alt text>":::
   <long description here>
:::image-end:::
```

:::image type="complex" source="<folderPath>" alt-text="<alt text>":::
   <long description here>
:::image-end:::


If `type="complex"`, `source`, `alt-text`, a long description, and the `:::image-end:::` tag are all required.

When your changes are in preview or published, you can check whether the long description exists by right-clicking on the image and selecting **Inspect** (when using Microsoft Edge browser, although other browsers have similar features). This action brings you to the image source in the HTML code, underneath which you'll find a *visually-hidden* class. Expand the dropdown on this class, and you'll find your long description:

:::image type="content" source="media/markdown-reference/long-description.png" alt-text="Screenshot of the HTML for an image and its long description.":::

### Automatic borders

The `:::image:::` extension also supports the `border` property, which  automatically adds a 1-pixel gray border around your image. The `border` property is `true` by default for `content` and `complex` images, so you'll get the border automatically unless you explicitly add the property with a value of `false`. The `border` property is `false` by default for `icon` images.

The `border` property is the recommended way to add a border. Don't create your own borders manually.

<!-- This section can be allowed publicly, but there's no external guide article for how to use lightboxes, so we can't add it until we have an external-guide equivalent.

### Creating an expandable image

The optional `lightbox` property allows you to create an expanded image, as described in [Create an expandable screenshot (lightbox)](contribute-how-to-use-lightboxes.md). The value of `lightbox` is the path to the expanded image.

-->

### Specifying loc-scope

Sometimes the localization scope for an image is different from that of the article or module that contains it. This can cause a bad global experience: for example, if a screenshot of a product is accidentally localized into a language the product isn't available in. To prevent this, you can specify the optional `loc-scope` attribute in images of types `content` and `complex`, and is required for screenshots that show a product with a different localization scope than the article or module that contains it.

### Icons

The image extension supports icons, which are decorative images and should not have alt text. The syntax for icons is:

```md
:::image type="icon" source="<folderPath>":::
```

If `type="icon"`, `source` should be specified but `alt-text` shouldn't be.

The `border` property  is `false` by default for icons. If your decorative image requires the standard image border, explicitly add `border="true"` to the `:::image:::` tag.

<!-- No lightbox article in external guide, so commenting this out for now.

The `lightbox` property works the same for `icon` images as for standard `content` images.

-->

## Included Markdown files

Where markdown files need to be repeated in multiple articles, you can use an include file. The includes feature instructs Microsoft Learn to replace the reference with the contents of the include file at build time. You can use includes in the following ways:

- Inline: Reuse a common text snippet inline with within a sentence.
- Block: Reuse an entire Markdown file as a block, nested within a section of an article.

An inline or block include file is a Markdown (.md) file. It can contain any valid Markdown. Include files are typically located in a common *includes* subdirectory, in the root of the repository. When the article is published, the included file is seamlessly integrated into it.

### Includes syntax

Block include is on its own line:

```md
[!INCLUDE [<title>](<filepath>)]
```

Inline include is within a line:

```md
Text before [!INCLUDE [<title>](<filepath>)] and after.
```

Where `<title>` is the name of the file and `<filepath>` is the relative path to the file. `INCLUDE` must be capitalized and there must be a space before the `<title>`.

Here are requirements and considerations for include files:

- Use block includes for significant amounts of content--a paragraph or two, a shared procedure, or a shared section. Don't use them for anything smaller than a sentence.
- Includes won't be rendered in the GitHub-rendered view of your article because they rely on Microsoft Learn extensions. They'll be rendered only after publication.
- Write all the text in an include file in complete sentences or phrases that don't depend on preceding or following text in the article that references the include. Ignoring this guidance creates an untranslatable string in the article.
- Don't embed include files within other include files.
- `/Includes` folders are excluded from build. Therefore, images stored in `/includes` folders and referenced in included files won't be displayed in published content. Store images in a `/media` folder outside the `/includes` folder.
- As with regular articles, don't share media between include files. Use a separate file with a unique name for each include and article. Store the media file in the media folder that's associated with the include.
- Don't use an include as the only content of an article.  Includes are meant to be supplemental to the content in the rest of the article.

## Indentation

In Markdown, spaces before the first character of a line determine the line's alignment relative to the preceding lines. Indentation especially influences numbered and bulleted lists to render multiple levels of nesting in a hierarchical or outline format.

To indent text to align with a preceding paragraph or an item in a numbered or bulleted list, use spaces.

The following two examples show how indented paragraphs render based on their relationship to other paragraphs.

```
1. This is a numbered list example (one space after the period before the letter T).
   This sentence is indented three spaces.
   This code block is indented three spaces.

- This is a bulleted list example (one space after the bullet before the letter T).
  This sentence is indented two spaces.
  > [!TIP]
  > This tip is indented two spaces.
  - This is a second-level bullet (indented two spaces, with one space after the bullet before the letter T).
    This sentence is indented four spaces.
    > This quote block is indented four spaces.
```

The example above renders as:

1. This is a numbered list example (one space after the period before the letter T).

   This sentence is indented three spaces.

   ```
   This code block is indented three spaces.
   ```

- This is a bulleted list example (one space after the bullet before the letter T).

  This sentence is indented two spaces.

  > [!TIP]
  > This tip is indented two spaces.
  - This is a second-level bullet (indented two spaces, with one space after the bullet before the letter T).

    This sentence is indented four spaces.

    > This quote block is indented four spaces.

## Links

For information on syntax for links, see [Use links in documentation](how-to-write-links.md).

## Lists (Numbered, Bulleted, Checklist)

### Numbered list

To create a numbered list, you can use all 1s. The numbers are rendered in ascending order as a sequential list when published. For increased source readability, you can increment your lists manually.

Don't use letters in lists, including nested lists. They don't render correctly when published to Microsoft Learn. Nested lists using numbers will render as lowercase letters when published. For example:

```md
1. This is
1. a parent numbered list
   1. and this is
   1. a nested numbered list
1. (fin)
```

This renders as follows:

1. This is
1. a parent numbered list
   1. and this is
   1. a nested numbered list
1. (fin)

### Bulleted list

To create a bulleted list, use `-` or `*` followed by a space at the beginning of each line:

```md
- This is
- a parent bulleted list
  - and this is
  - a nested bulleted list
- All done!
```

This renders as follows:

- This is
- a parent bulleted list
  - and this is
  - a nested bulleted list
- All done!

Whichever syntax you use, `-` or `*`, use it consistently within an article.

### Checklist

Checklists are available for use on Microsoft Learn via a custom Markdown extension:

```md
> [!div class="checklist"]
> * List item 1
> * List item 2
> * List item 3
```

This example renders on Microsoft Learn like this:

> [!div class="checklist"]
> * List item 1
> * List item 2
> * List item 3

Use checklists at the beginning or end of an article to summarize "What will you learn" or "What have you learned" content. Do not add random checklists throughout your articles.

## Next step action

You can use a custom extension to add a next step action button to Microsoft Learn pages.

The syntax is as follows:

```markdown
> [!div class="nextstepaction"]
> [button text](link to topic)
```

For example:

```md
> [!div class="nextstepaction"]
> [Learn about adding code to articles](code-in-docs.md)
```

This renders as follows:

> [!div class="nextstepaction"]
> [Learn about adding code to articles](code-in-docs.md)

You can use any supported link in a next step action, including a Markdown link to another web page. In most cases, the next action link will be a relative link to another file in the same docset.

## Non-localized strings

You can use the custom `no-loc` Markdown extension to identify strings of content that you would like the localization process to ignore.

All strings called out will be case-sensitive; that is, the string must match exactly to be ignored for localization.

To mark an individual string as non-localizable, use this syntax:

```markdown
:::no-loc text="String":::
```

For example, in the following, only the single instance of `Document` will be ignored during the localization process:

```md
# Heading 1 of the Document

Markdown content within the :::no-loc text="Document":::.  The are multiple instances of Document, document, and documents.
```

```

You can also use metadata in the YAML header to mark all instances of a string within the current Markdown file as non-localizable:

```yml
author: cillroy
no-loc: [Global, Strings, to be, Ignored]
```

> [!NOTE]
> The no-loc metadata is not supported as global metadata in *docfx.json* file. The localization pipeline doesn't read the *docfx.json* file, so the no-loc metadata must be added into each individual source file.

In the following example, both in the metadata `title` and the Markdown header the word `Document` will be ignored during the localization process.

In the metadata `description` and the Markdown main content the word `document` is localized, because it does not start with a capital `D`.

```md
---
title: Title of the Document
author: author-name
description: Description for the document
no-loc: [Title, Document]
---
# Heading 1 of the Document

Markdown content within the document.
```

<!-- commenting out for now because no one knows what this means
## Section definition

You might need to define a section. This syntax is mostly used for code tables.
See the following example:

````
> [!div class="tabbedCodeSnippets" data-resources="OutlookServices.Calendar"]
> ```cs
> <cs code text>
> ```
> ```javascript
> <js code text>
> ```
````

The preceding blockquote Markdown text will be rendered as:
> [!div class="tabbedCodeSnippets" data-resources="OutlookServices.Calendar"]
> ```cs
> <cs code text>
> ```
> ```javascript
> <js code text>
> ```
-->




   > Use `\` to escape special characters:
   > ```markdown
   > Lorem :::no-loc text="Find a \"Quotation\""::: Ipsum.
   > ```


## Selectors

Selectors are UI elements that let the user switch between multiple flavors of the same article. They are used in some docsets to address differences in implementation across technologies or platforms. Selectors are typically most applicable to our mobile platform content for developers.

Because the same selector Markdown goes in each article file that uses the selector, we recommend placing the selector for your article in an include file. Then you can reference that include file in all your article files that use the same selector.

There are two types of selectors: a single selector and a multi-selector.

### Single selector

```md
> [!div class="op_single_selector"]
> - [Universal Windows](../articles/notification-hubs-windows-store-dotnet-get-started/)
> - [Windows Phone](../articles/notification-hubs-windows-phone-get-started/)
> - [iOS](../articles/notification-hubs-ios-get-started/)
> - [Android](../articles/notification-hubs-android-get-started/)
> - [Kindle](../articles/notification-hubs-kindle-get-started/)
> - [Baidu](../articles/notification-hubs-baidu-get-started/)
> - [Xamarin.iOS](../articles/partner-xamarin-notification-hubs-ios-get-started/)
> - [Xamarin.Android](../articles/partner-xamarin-notification-hubs-android-get-started/)
```

... will be rendered like this:

> [!div class="op_single_selector"]
> - [Universal Windows](how-to-write-links.md)
> - [Windows Phone](how-to-write-links.md)
> - [iOS](how-to-write-links.md)
> - [Android](how-to-write-links.md)
> - [Kindle](how-to-write-links.md)
> - [Baidu](how-to-write-links.md)
> - [Xamarin.iOS](how-to-write-links.md)
> - [Xamarin.Android](how-to-write-links.md)

### Multi-selector

```md
> [!div class="op_multi_selector" title1="Platform" title2="Backend"]
> - [(iOS | .NET)](./mobile-services-dotnet-backend-ios-get-started-push.md)
> - [(iOS | JavaScript)](./mobile-services-javascript-backend-ios-get-started-push.md)
> - [(Windows universal C# | .NET)](./mobile-services-dotnet-backend-windows-universal-dotnet-get-started-push.md)
> - [(Windows universal C# | Javascript)](./mobile-services-javascript-backend-windows-universal-dotnet-get-started-push.md)
> - [(Windows Phone | .NET)](./mobile-services-dotnet-backend-windows-phone-get-started-push.md)
> - [(Windows Phone | Javascript)](./mobile-services-javascript-backend-windows-phone-get-started-push.md)
> - [(Android | .NET)](./mobile-services-dotnet-backend-android-get-started-push.md)
> - [(Android | Javascript)](./mobile-services-javascript-backend-android-get-started-push.md)
> - [(Xamarin iOS | Javascript)](./partner-xamarin-mobile-services-ios-get-started-push.md)
> - [(Xamarin Android | Javascript)](./partner-xamarin-mobile-services-android-get-started-push.md)
```

... will be rendered like this:

> [!div class="op_multi_selector" title1="Platform" title2="Backend"]
> - [(iOS | .NET)](how-to-write-links.md)
> - [(iOS | JavaScript)](how-to-write-links.md)
> - [(Windows universal C# | .NET)](how-to-write-links.md)
> - [(Windows universal C# | Javascript)](how-to-write-links.md)
> - [(Windows Phone | .NET)](how-to-write-links.md)
> - [(Windows Phone | Javascript)](how-to-write-links.md)
> - [(Android | .NET)](how-to-write-links.md)
> - [(Android | Javascript)](how-to-write-links.md)
> - [(Xamarin iOS | Javascript)](how-to-write-links.md)
> - [(Xamarin Android | Javascript)](how-to-write-links.md)

## Subscript and superscript

You should only use subscript or superscript when necessary for technical accuracy, such as when writing about mathematical formulas. Don't use them for non-standard styles, such as footnotes.

For both subscript and superscript, use HTML:

```html
Hello <sub>This is subscript!</sub>
```

This renders as follows:

Hello <sub>This is subscript!</sub>

```html
Goodbye <sup>This is superscript!</sup>
```

This renders as follows:

Goodbye <sup>This is superscript!</sup>

## Tables

The simplest way to create a table in Markdown is to use pipes and lines. To create a standard table with a header, follow the first line with dashed line:

```md
|This is   |a simple   |table header|
|----------|-----------|------------|
|table     |data       |here        |
|it doesn't|actually   |have to line up nicely!|
```

This renders as follows:

|This is   |a simple   |table header|
|----------|-----------|------------|
|table     |data       |here        |
|it doesn't|actually   |have to line up nicely!|

You can align the columns by using colons:

```md
| Fun                  | With                 | Tables          |
| :------------------- | -------------------: |:---------------:|
| left-aligned column  | right-aligned column | centered column |
| $100                 | $100                 | $100            |
| $10                  | $10                  | $10             |
| $1                   | $1                   | $1              |
```

Renders as follows:

| Fun                  | With                 | Tables          |
| :------------------- | -------------------: |:---------------:|
| left-aligned column  | right-aligned column | centered column |
| $100                 | $100                 | $100            |
| $10                  | $10                  | $10             |
| $1                   | $1                   | $1              |

> [!TIP]
> The Learn Authoring Extension for VS Code makes it easy to add basic Markdown tables!
>
> You can also use an [online table generator](http://www.tablesgenerator.com/markdown_tables).

### Line breaks within words in any table cell

Long words in a Markdown table might make the table expand to the right navigation and become unreadable. You can solve that by allowing rendering to automatically insert line breaks within words when needed. Just wrap up the table with the custom class `[!div class="mx-tdBreakAll"]`.

Here is a Markdown sample of a table with three rows that will be wrapped by a `div` with the class name `mx-tdBreakAll`.

```md
> [!div class="mx-tdBreakAll"]
> |Name|Syntax|Mandatory for silent installation?|Description|
> |-------------|----------|---------|---------|
> |Quiet|/quiet|Yes|Runs the installer, displaying no UI and no prompts.|
> |NoRestart|/norestart|No|Suppresses any attempts to restart. By default, the UI will prompt before restart.|
> |Help|/help|No|Provides help and quick reference. Displays the correct use of the setup command, including a list of all options and behaviors.|
```

It will be rendered like this:

> [!div class="mx-tdBreakAll"]
> |Name|Syntax|Mandatory for silent installation?|Description|
> |-------------|----------|---------|---------|
> |Quiet|/quiet|Yes|Runs the installer, displaying no UI and no prompts.|
> |NoRestart|/norestart|No|Suppresses any attempts to restart. By default, the UI will prompt before restart.|
> |Help|/help|No|Provides help and quick reference. Displays the correct use of the setup command, including a list of all options and behaviors.|

### Line breaks within words in second column table cells

You might want line breaks to be automatically inserted within words only in the second column of a table. To limit the breaks to the second column, apply the class `mx-tdCol2BreakAll` by using the `div` wrapper syntax as shown earlier.

### Inconsistent column widths between tables

You may notice that the column widths of the tables in your articles look odd or inconsistent. This behavior occurs because the length of text within the cells determines the layout of the table. Unfortunately, there's no way to control how the tables render. This is a limitation of Markdown. Even though it would look nicer to have the width of table columns be consistent, this would have some disadvantages too:

- Interlacing HTML code with Markdown makes topics more complicated and discourages community contributions.
- A table that you make look good for a specific screen size may end up looking unreadable at different screen sizes as it preempts responsive rendering.

### Data matrix tables

A data matrix table has both a header and a weighted first column, creating a matrix with an empty cell in the top left. Microsoft Learn has custom Markdown for data matrix tables:

```md
|                  |Header 1 |Header 2|
|------------------|---------|--------|
|**First column A**|Cell 1A  |Cell 2A |
|**First column B**|Cell 1B  |Cell 2B |
```

The example renders as:

|                  |Header 1 |Header 2|
|------------------|---------|--------|
|**First column A**|Cell 1A  |Cell 2A |
|**First column B**|Cell 1B  |Cell 2B |

Every entry in the first column must be styled as bold (`**bold**`); otherwise the tables won't be accessible for screen readers or valid for Microsoft Learn.

> [!TIP]
> The Learn Authoring Pack for VS Code includes a function to convert a regular Markdown table into a data matrix table. Just select the table, right-click, and select **Convert to data matrix table**.

### HTML Tables

HTML tables aren't recommended for Microsoft Learn. They aren't human readable in the source - which is a key principle of Markdown.

---
author: mammerla
ms.author: v-bbortree
title: Entity Documentation - minecraft:dweller
ms.prod: gaming
---

# Entity Documentation - minecraft:dweller

`minecraft:dweller` allows a mob to join and migrate between villages and other dwellings.

## Parameters

|Name |Default Value  |Type  |Description  |
|:----------|:----------|:----------|:----------|
| dwelling_type| *not_set*| String| The type of dwelling the mob wishes to join. Current Types: village |
|dweller_role|*not set* | String |  The role of which the mob plays in the dwelling. Current Roles: inhabitant, defender, hostile, passive. |
|update_interval_base|*not set* | Decimal| How often the mob checks on their dwelling status in ticks. Positive values only. |
|update_interval_variant|*not set* | Decimal|  The variant value in ticks that will be added to the update_interval_base. |
|can_find_poi| *not set*| Boolean|  Whether or not the mob can find and add POI's to the dwelling. |
|first_founding_reward| *not set*| Integer|  How much reputation should the players be rewarded on first founding? |
|can_migrate| *not set*| Boolean| Can this mob migrate between dwellings? Or does it only have its initial dwelling? |
|dwelling_bounds_tolerance| *not set*| Float | A padding distance for checking if the mob is within the dwelling. |
|preferred_profession| *not set*| String| Allows the user to define a starting profession for this particular Dweller, instead of letting them choose organically. (They still need to gain experience from trading before this takes effect.) |



## Example

```json
"minecraft:dweller": {
    "dwelling_type": "village",
    "dweller_role": "inhabitant",
    "preferred_profession": "toolsmith",
    "update_interval_base": 60,
    "update_interval_variant": 40,
    "can_find_poi": true,
    "can_migrate": true,
    "first_founding_reward": 5
}
```

## Vanilla entities examples

### villager_v2

```json
"minecraft:dweller": {
    "dwelling_type": "  ",
    "dweller_role": "inhabitant",
    "preferred_profession": "farmer",
    "update_interval_base": 60,
    "update_interval_variant": 40,
    "can_find_poi": true,
    "can_migrate": true,
    "first_founding_reward": 5
}
```

## Vanilla entities using `minecraft:dweller`

- [cat](../../../../Source/VanillaBehaviorPack_Snippets/entities/cat.md)
- [evocation_illager](../../../../Source/VanillaBehaviorPack_Snippets/entities/evocation_illager.md)
- [iron_golem](../../../../Source/VanillaBehaviorPack_Snippets/entities/iron_golem.md)
- [evocation_illager](../../../../Source/VanillaBehaviorPack_Snippets/entities/evocation_illager.md)
- [pillager](../../../../Source/VanillaBehaviorPack_Snippets/entities/pillager.md)
- [ravager](../../../../Source/VanillaBehaviorPack_Snippets/entities/ravager.md)
- [villager_v2](../../../../Source/VanillaBehaviorPack_Snippets/entities/villager_v2.md)
- [vindicator](../../../../Source/VanillaBehaviorPack_Snippets/entities/vindicator.md)
- [witch](../../../../Source/VanillaBehaviorPack_Snippets/entities/witch.md)

---
author: kakinnun
ms.author: kakinnun
title: 1.19.70 Update Notes
ms.prod: gaming
description: Update summary of Creator changes in Bedrock 1.19.70
---
# Minecraft Bedrock 1.19.70 Update Notes for Creators

Minecraft Bedrock has been updated to 1.19.70 and there are a number of changes of note for add-on creators.

## Holiday Creator Features ##

We continue to work on bringing the Holiday Creator Features out of experimental. Our current focus is on block components. The following block components are now available outside of the experimental toggle in 1.19.70.

- [Block Properties and Permutations](../Reference/Content/BlockReference/Examples/BlockPropertiesAndPermutations.md)

## Components ##

We've removed support for field "data" in the following commands:

- /clone
- /execute
- /fill
- /setblock
- /testforblock

**Example:**

No longer supported:

```/setblock ~ ~ ~ minecraft:wool 1```

Supported:

```/setblock ~ ~ ~ minecraft:wool ["color":"orange"]```

## Creator API ##

A reminder that a limited number of APIs were released out of experimental in [1.19.50](Update1.19.50.md). There are no new APIs leaving experimental in 1.19.70 but there are a number of new APIs behind experimental.

To use Beta APIs, you need to enable the Beta APIs experimental toggle.

**IMPORTANT BREAKING CHANGE:** The classes Location and BlockLocation no longer exist in the beta script API. All usages of these classes have been changed to use the Vector3 interface (that is, { x: 1, y: 2, z: 3} objects).

**BlockPermutation**

BlockPermutation has been significantly refactored! Every BlockPermutation now share a unique JavaScript handle so exact equality (===) will work for permutations that share exactly the same state values. We've also added utility methods that make interacting with permutations easier, which includes the removal of the XBlockProperty classes and now directly return properties (boolean | number | string) or a while collection of properties ( Record<string, boolean | number | string>).

Beta APIs will continue to be developed behind the Beta API experimental flag so if you want to use those, make sure you have that flag enabled and your manifest.json references will need to update to 1.1.0-beta. Read more about [script versioning](ScriptVersioning.md).

Looking ahead to upcoming releases, we are looking to release our next set of APIs out of experimental, potentially including read-only block and entity APIs.

Scripting is a powerful way to add complex behavior to your experience. It allows for a [professional development environment that includes profiling and hot reloading](./ScriptDeveloperTools.md).

[Get started with scripting](https://aka.ms/startwithmcscript).
---
title: Features
description: Adding Xbox Live features to your game, such as Identity, Social features, Achievements, Cloud Storage, and Multiplayer features; and configuring Custom services to use Xbox Live.
kindex: Features
ms.topic: navigation
layout: LandingPage
ms.localizationpriority: high
author: mikehoffms
ms.author: v-mihof
ms.date: 11/01/2019
---

# Features


### In this section

|     |     |
| --- | --- |
| [Identity](identity/live-identity-nav.md) | User profile, authentication and sign-in, privileges, and privacy settings. |
| [Player Data](player-data/live-playerdata-nav.md) | Achievements, player stats, leaderboards, and featured stats. |
| [Social](social/live-social-nav.md) | Friends list (People system), official and in-game clubs, activity feed (presence strings), and reputation. |
| [Multiplayer](multiplayer/live-multiplayer-nav.md) | Matchmaking, invites, voice and text chat, and networking. |
| [Cloud Storage](cloud-storage/live-cloud-storage-nav.md) | Includes Connected Storage to store game state, and Title Storage to store player statistics and assets. |
| [Custom services](custom-services/live-custom-services-nav.md) | Using Xbox Live with your own Web services; and configuring relying parties, single sign-on, and access policies. |
| [General features](general/live-general-nav.md) | RTA (subscribes to state data, user statistics, and presence), TCUI (shows pre-defined user interface displays), and Xbox Live samples. |


### See also

* [Features supported for each developer program](../get-started/join-dev-program/live-feature-comparison-table.md)

---
author: mammerla
ms.author: v-jillheaden
title: Popular Commands
ms.prod: gaming
description: "A guide covering many of the most common commands that are used within Minecraft: Bedrock Edition"
---

# Popular Commands

There are a number of commands that are vital to many command systems. This list will go over the more popular and useful commands that are likely to be used in some fashion.

In this tutorial you will learn the following:

> [!div class="checklist"]
>
> - Some of the more popular commands to use when starting out.

### Requirements

It's recommended that the following be completed before beginning this tutorial.

- [Introduction to Commands](CommandsIntroduction.md)
- [Getting Started with Command Blocks](CommandBlocks.md)

## /help

First and foremost is the `/help` command.

```
/help <page: int>
```

Running `/help` in the chat by itself will list every command you can run. However, since there are so many, they have been divided up into several pages, where you are shown only one page at a time. Supplying a page number will change which page of commands to show.

Whenever you want to know the usage and all of the syntaxes of a command, you can use `/help` followed by the name of the command. This can be helpful if you want an overview of the command.

```
/help [command: CommandName]
```

## /effect

Adding and removing status effects is done with the `/effect` command. All status effects on the target can also be cleared.

## /execute

The primary function of the `/execute` command is to modify the executor and execution origin of a nested command. However, its syntax allows for some conditional command execution that would otherwise be missing in functions.

```
/execute <origin: target> <position: x y z> <command: command>

/execute <origin: target> <position: x y z> detect <detectPos: x y z> <block: Block> <data: int> <command: command>
```

For example, to detect if there's a specific block beneath the player and run a command as a result, the "detect" argument can be used instead of having a nested `/testforblock` command.

```
/execute @a ~ ~ ~ detect ~ ~-1 ~ grass 0 say Player is standing on top of grass.
```

## /gamemode

When developing content, most often you'll be playing in creative mode. However, testing your content may require you to enter survival or adventure mode. The `/gamemode` command allows you to change your current game mode.

```
/gamemode <gameMode: GameMode> [player: target]
/gamemode <gameMode: int> [player: target]
```

You have several options for the game mode: the full name, a single character, or a numeric representation. The accepted values are "survival" ("s" or 0), "creative" ("c" or 1), and "adventure" ("a" or 2). There is also "default" ("d"), which sets your game mode to whatever the world's default game mode is. The following will set your own game mode to creative when you run it in the chat.

```
/gamemode creative
```

You can also change the game mode of a specific player using target selectors, which can be helpful when controlling gameplay aspects (such as the inability to mine anything in adventure mode). The following changes the game mode of all players with the "sometag" tag to adventure mode.

```
/gamemode adventure @a[tag=sometag]
```

## /gamerule

This `/gamerule` command changes specific gameplay aspects and has options helpful for development. There are a large number of gamerules, which can be listed using the chat's auto-complete feature.

```
/gamerule <rule: BoolGameRule> [value: Boolean]
/gamerule <rule: IntGameRule> [value: int]
```

Some gamerules accept only true or false as the value of the rule ("BoolGameRule") while others only accept integers ("IntGameRule"). The primary gamerules crucial for development, which are all booleans, are "commandblockoutput", "sendcommandfeedback", and "commandblocksenabled", "dodaylightcycle", and "doweathercycle" listed below.

### "commandblockoutput"

You may have noticed that all of the previous command examples send an output to the chat. This can be a nuisance, especially for commands running every tick. The "commandblockoutput" gamerule, when set to false, will disable that chat output.

```
/gamerule commandblockoutput false
```

### "sendcommandfeedback"

Disabling command block output does not disable all feedback. There are some commands, such as `/xp`, that will continue providing feedback even when command block output is disabled. The "sendcommandfeedback" gamerule will disable these messages as well, allowing for a completely silent command system.

```
/gamerule sendcommandfeedback false
```

### "commandblocksenabled"

To quickly enable or disable command blocks in the world, the "commandblocksenabled" can be toggled on and off. This can be especially helpful if a repeating command block is repeatedly teleporting you.

```
/gamerule commandblocksenabled false
```

### "dodaylightcycle"

Either for development or for gameplay concerns, you can choose to prevent the daylight cycle from moving. This means that whatever time you set the game to (such as with the [`/time` command](#time-set)), it will stay at that time.

```
/gamerule dodaylightcycle false
```

### "doweathercycle"

As with "dodaylightcycle", you may also want to control the weather cycle. If you intend to have a happy setting, a thunderstorm setting in may not achieve the best effect. When "doweathercycle" is disabled, the weather will stay as it is, including when set with the [`/weather` command](#weather).

```
/gamerule doweathercycle false
```

## /give, /clear, /replaceitem

This trio of commands manipulate the player's inventory, though `/replaceitem` can also modify non-player entity inventories and blocks with inventories. The `/give` command can provide items, the `/clear` command can remove items, and the `/replaceitem` command can place items in specific slots in the inventory.

## /locate

Finds the nearest specified biome or structure if it exists in the current dimension. Prints an error if it does not.

```
/locate biome <biome name>
/locate structure <structure name>
```

The locate command takes two arguments: the first argument specifies whether to locate a biome or a structure, the second argument specifies the name of the biome or structure to be located.

```
/locate biome beach
/locate structure village
```

## /scoreboard

The `/scoreboard` command is a powerful method of keeping track of numeric values on a per-entity basis, as well as performing mathematical operations with commands. The first step is creating an objective and (optionally) displaying it on the sidebar.

```
/scoreboard objectives add objectiveA dummy

/scoreboard objectives setdisplay sidebar objectiveA
```

The simplest course of action would be rewarding the player with a point if they accomplish some task.

```
/scoreboard players add @p objectiveA 1
```

Afterwards, target selectors can be used to select players who achieve a certain number of points.

```
@a[scores={objectiveA=10..}]
```

## /setblock, /fill, /clone

These commands change the physical blocks in the world. The `/setblock` command can set a single block while the `/fill` command can set multiple of the same block. The `/clone` command, on the other hand, will take a copy of blocks from one area and paste it into another.

## /summon

The `/summon` command is used to spawn a new entity into the world, from cows to sheep to your own custom entities.

## /setworldspawn

If you have a specific location that you want players new to the world or players who die to spawn at, the `/setworldspawn` command provides that ability.

```
/setworldspawn [spawnPoint: x y z]
```

Note that players who die after they have set their spawn with a bed will still respawn at their bed.

## /tag

Similar to `/scoreboard`, the `/tag` command allows you to keep track of string values on a per-entity basis. Tags would be used when a numeric value is not needed, such as for "true or false" situations. For example, you could tag entities as being a boss and later target those same entities based on that tag.

```
/tag @e[type=sheep] add boss
/tag @e[type=minecart] add boss
```

```
/say Bosses: @e[tag=boss]
```

## /tellraw, /titleraw

The `/say`, `/tell`, and `/title` commands are not ideal when presenting information as they are not open to translation. The `/tellraw` and `/titleraw` commands can be translated using a JSON input for the message. The `/tellraw` command is also clearer in intent as the message is not accompanied with a "whisper" statement.

```
/tellraw @a {"rawtext":[{"translate":"commands.testfor.success","with":["PlayerName"]}]}

/tellraw @a {"rawtext":[{"text":"Hello World"}]}
```

## /testfor, /testforblock, /testforblocks

These commands test for the existence of an entity, block, and a copy of a block structure. While these commands can be useful alongside conditional command blocks, they are less useful in functions as there is no equivalent conditional setting in functions.

Both the `/testfor` and `/testforblock` commands can generally be skipped over in favor of `/execute`, which supports running a command based on the existence of an entity and running a command based on the existence of a block.

## /time set

Changing the gameplay environment can be essential for providing the right atmosphere. Changing the time of day is one method of doing so.

```
/time set <amount: int>
/time set <time: TimeSpec>
```

You can either provide an integer which represents a precise time of the day, or "TimeSpec" can be set to one of the following values to more easily select common times of day: "day", "midnight", "night", "noon", "sunrise", and "sunset". For example, quickly setting the sun directly above the player would use the following:

```
/time set noon
```

## /tp or /teleport

Sometimes you may want to provide an easy means of transportation or need a form of controlling where the player (or even non-player entity) is. The `/tp` command will teleport targeted players to specific locations, or even to other entities. The syntaxes for `/tp` may look daunting at first, but note many branch to and from the same options.

```
/tp <victim: target> <destination: target> [checkForBlocks: Boolean]
/tp <victim: target> <destination: x y z> [checkForBlocks: Boolean]
/tp <victim: target> <destination: x y z> [yRot: value] [xRot: value] [checkForBlocks: Boolean]
/tp <victim: target> <destination: x y z> facing <lookAtEntity: target> [checkForBlocks: Boolean]
/tp <victim: target> <destination: x y z> facing <lookAtPosition: x y z> [checkForBlocks: Boolean]
```

At its simplest, you can teleport the player to a specific coordinate location.

```
/tp @p 100 50 100
```

You can also teleport them to another player or entity. The destination target must resolve to a single entity, so you have to use a target selector that can only target one entity or ensure there are no duplicates of the intended target (such as using the [`/tag` command](#tag) to identify targets).

```
/tp Steve Alex
/tp @p @e[type=minecraft:armor_stand,c=1]
/tp @a @e[type=minecraft:creeper,tag=destination_1]
```

The common option "checkForBlocks" defaults to false, but when it is set to true, it will prevent teleporting the player if the location they would end up in is occupied by blocks.

This command teleports Steve to coordinates [50, 63, 50] and makes sure there are no blocks in the way.

```
/tp Steve 50 63 50 true
```

> [!NOTE]
> One of the differences between using commands and commands blocks is that `/tp @s ...` will not work in a command block.

## /weather

Like [`/time set`](#time-set), the `/weather` command can be used to change the environment. If you want a specific type of weather to occur, this would be the command you would use.

```
/weather <clear|rain|thunder> [duration: int]
```

The optional duration is the number of game ticks that the selected weather will last for. 20 game ticks is one second, so for each second you want the weather to last, multiply it by twenty. The following sets the weather to thunder for 30 seconds, which is 600 ticks.

```
/weather thunder 600
```

Be sure that the ["doweathercycle" gamerule](#doweathercycle) is false if you intend on making use of the duration. If it is true, the weather cycle will not occur, rendering the duration useless.

## What's Next?

After learning about commands and target selectors, command blocks, and a variety of useful commands, it's time to put them together and create a small project in the form of a Complete the Monument system.

> [!div class="nextstepaction"]
> [How to Make a Complete the Monument World](CommandsHowToMakeACTMWorld.md)
---
author: mammerla
ms.author: mikeam
title: Recipe Documentation - Potion Brewing Container Recipe
ms.prod: gaming
---

# Recipe Documentation - Potion Brewing Container Recipe

Represents a Potion Brewing Container Recipe.

## Parameters

|Name |Type |Description |
|:-----------|:-----------|:-----------|
|input| potion| Input potion used in the brewing container recipe. |
|output| potion| Output potion from the brewing container recipe. |
|reagent| item| Item used in the brewing container recipe with the input potion. |
|tags|String array | Item that can create the Brewing Container Recipe, such as "brewing_stand". |

## Potion Brewing Container Recipe Example

```JSON
{
"format_version": "1.17",
    "minecraft:recipe_brewing_container": {
    "description": {
        "identifier": "minecraft:brew_potion_sulphur"
    },
    "tags": [ "brewing_stand" ],
    "input": "minecraft:potion",
    "reagent": "minecraft:gunpowder",
    "output": "minecraft:splash_potion"
    }
}
```

## Vanilla Example

### Brew Potion Sulphur

:::code language="json" source="../../../../Source/VanillaBehaviorPack/recipes/brew_potion_sulphur.json":::
