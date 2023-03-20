# Microsoft Learn Markdown Parser
This repository contains code for a parser that converts Microsoft Learn markdown format into tokens. The main struct is MsMarkdown, which contains a vector of MsMarkdownToken enums.

Each MsMarkdownToken represents a different type of element in the markdown document. For example, there are tokens for metadata, comments, headings, code blocks, tables and more.

The parser can be used to convert Microsoft Learn markdown documents into a structured format that can be easily manipulated and analyzed.
