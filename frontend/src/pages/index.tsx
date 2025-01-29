import "@uiw/react-md-editor/markdown-editor.css";
import "@uiw/react-markdown-preview/markdown.css";
import React, { useState } from 'react';
import dynamic from 'next/dynamic';
import { Button, Container, Grid } from 'semantic-ui-react';
import rehypeSanitize from "rehype-sanitize";
import { getCodeString } from 'rehype-rewrite';
import katex from 'katex';
import 'katex/dist/katex.css';

const MDEditor = dynamic(() => import('@uiw/react-md-editor'), { ssr: false });

const MarkdownEditorPage: React.FC = () => {
  const [markdownContent, setMarkdownContent] = useState<string | undefined>('test');

  const handleConfirm = async () => {
    
  };

  return (
    <Container style={{ marginTop: '2rem', padding: '1rem' }}>
      <h1 style={{ textAlign: 'center', fontSize: '1.5rem' }}>DPB</h1>
      <Grid centered>
        <Grid.Column mobile={16} tablet={12} computer={8}>
            <MDEditor
              value={markdownContent}
              onChange={setMarkdownContent}
              previewOptions={{
                components: {
                  code: ({ children = [], className, ...props }) => {
                    if (typeof children === 'string' && /^\$\$(.*)\$\$/.test(children)) {
                      const html = katex.renderToString(children.replace(/^\$\$(.*)\$\$/, '$1'), {
                        throwOnError: false,
                      });
                      return <code dangerouslySetInnerHTML={{ __html: html }} style={{ background: 'transparent' }} />;
                    }
                    const code = props.node && props.node.children ? getCodeString(props.node.children) : children;
                    if (
                      typeof code === 'string' &&
                      typeof className === 'string' &&
                      /^language-katex/.test(className.toLocaleLowerCase())
                    ) {
                      const html = katex.renderToString(code, {
                        throwOnError: false,
                      });
                      return <code style={{ fontSize: '150%' }} dangerouslySetInnerHTML={{ __html: html }} />;
                    }
                    return <code className={String(className)}>{children}</code>;
                  },
                },
                rehypePlugins: [[rehypeSanitize]],
              }}
              style={{height: "100%"}}
            />
            <Button
              primary
              style={{ marginTop: '1rem', width: '100%' }}
              onClick={handleConfirm}
            >
              Submit
            </Button>
        </Grid.Column>
      </Grid>
    </Container>
  );
};

export default MarkdownEditorPage;
