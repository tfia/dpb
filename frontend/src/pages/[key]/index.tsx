import "@uiw/react-md-editor/markdown-editor.css";
import "@uiw/react-markdown-preview/markdown.css";
import React, { useState, useEffect } from 'react';
import MarkdownPreview from '@uiw/react-markdown-preview';
import { Button, Container, Dropdown, DropdownHeader, Grid, Header, Icon, Input, Placeholder, Segment, AccordionTitle, AccordionContent, Accordion, Divider, TextArea, Form } from 'semantic-ui-react';
import rehypeSanitize from "rehype-sanitize";
import { getCodeString } from 'rehype-rewrite';
import katex from 'katex';
import 'katex/dist/katex.css';
import { Helmet } from "react-helmet";
import { useRouter } from "next/router";
import { request } from '@/utils/network';
import toast from "react-hot-toast";
import { BACKEND_URL } from "@/constants/string";

const Home: React.FC = () => {
  const [markdownContent, setMarkdownContent] = useState<string | undefined>('');
  const [title, setTitle] = useState<string>('');
  const [createTime, setCreateTime] = useState<string>('');
  const [expireTime, setExpireTime] = useState<string>('');
  const [loading, setLoading] = useState<boolean>(true);
  const [showSource, setShowSource] = useState<boolean>(false);

  const router = useRouter();
  const { key } = router.query;

  useEffect(() => {
    if (key) {
      request(`${BACKEND_URL}/query/${key}`, 'GET')
        .then((data) => {
          setMarkdownContent(data.content);
          setTitle(data.title);
          let create_date = new Date(data.created_at);
          setCreateTime(create_date.toLocaleString());
          if (data.expire_at) {
            let expire_date = new Date(data.expire_at);
            setExpireTime(expire_date.toLocaleString());
          }
          setLoading(false);
        })
        .catch((error) => {
          router.push('/');
        });
    }
  }, [key]);

  return (
    <Container style={{ padding: '1rem', textAlign: 'center' }}>
      <Segment style={{ padding: '1.5rem', marginTop: '1rem', marginBottom: '1rem' }}>
        {loading ? (
          <Placeholder>
            <Helmet>
              <title>Loading... - DPB</title>
            </Helmet>
            <Placeholder.Header>
              <Placeholder.Line />
              <Placeholder.Line />
            </Placeholder.Header>
            <Placeholder.Paragraph>
              <Placeholder.Line />
              <Placeholder.Line />
              <Placeholder.Line />
              <Placeholder.Line />
              <Placeholder.Line />
              <Placeholder.Line />
            </Placeholder.Paragraph>
          </Placeholder>
        ) :
          (<Grid centered>
            <Helmet>
              <title>{title} - DPB</title>
            </Helmet>
            <Grid.Column mobile={16} tablet={16} computer={16}>
              <Header as='h1'>{title}</Header>
              <p>
                <Icon name="calendar outline" />Created at {createTime} / <Icon name="hourglass half" />{expireTime ? `Will expire at ${expireTime}` : `Never expires`}
              </p>
              <Divider />
              <MarkdownPreview
                source={markdownContent}
                components={{
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
                }}
                rehypePlugins={[[rehypeSanitize]]}
                style={{ marginBottom: '15px' }}
              />
              <Accordion styled fluid>
                <AccordionTitle
                  active={showSource}
                  index={0}
                  onClick={() => setShowSource(!showSource)}
                >
                  <Icon name='dropdown' />
                  Source Code
                </AccordionTitle>
                <AccordionContent active={showSource}>
                  <Segment secondary style={{ position: 'relative' }}>
                    <Button
                      circular
                      icon='copy outline'
                      style={{ position: 'absolute', right: 0, top: '5px' }}
                      onClick={() => {
                        navigator.clipboard.writeText(markdownContent || '');
                        toast.success('Copied to clipboard!');
                      }}
                    />
                    <p style={{ whiteSpace: 'pre-wrap' }}>
                      {markdownContent}
                    </p>
                  </Segment>
                </AccordionContent>
              </Accordion>
            </Grid.Column>
          </Grid>)}
      </Segment>
    </Container>
  );
};

export default Home;
