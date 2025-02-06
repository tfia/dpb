export const BACKEND_URL = process.env.NEXT_PUBLIC_API_BASE_URL;

export const MD_GUIDE = `
#### **Headings**  
\`\`\`markdown
# Heading 1
## Heading 2
### Heading 3
\`\`\`
**Preview:**  
# Heading 1  
## Heading 2  
### Heading 3  

#### **Bold & Italics**  
\`\`\`markdown
**Bold**  
*Italic*  
~~Strikethrough~~
\`\`\`
**Preview:**  
**Bold**  
*Italic*  
~~Strikethrough~~  

#### **Lists**  
- Unordered:  
  \`\`\`markdown
  - Item 1  
  - Item 2  
  \`\`\`
- Ordered:  
  \`\`\`markdown
  1. First  
  2. Second  
  \`\`\`

#### **Links & Images**  
\`\`\`markdown
[Link](https://example.com)  
![Alt text](image_url)
\`\`\`

#### **Code Blocks**  
- Inline: \`\` \`code\` \`\`  
- Block:  
  \`\`\`markdown
  \`\`\`js
  console.log("Hello, world!");
  \`\`\`
  \`\`\`

#### **Blockquotes & Tables**  
\`\`\`markdown
> This is a quote.  

| Name  | Age |
|-------|-----|
| Alice | 24  |
| Bob   | 30  |
\`\`\`

#### **KaTeX (Math Support)**  
- Inline:  
  \`\`\`markdown
  \`$$c = \\pm\\sqrt{a^2 + b^2}$$\`
  \`\`\`
- Block:  
  \`\`\`markdown
  \`\`\`KaTeX
  c = \\pm\\sqrt{a^2 + b^2}
  \`\`\`
  \`\`\`
`;