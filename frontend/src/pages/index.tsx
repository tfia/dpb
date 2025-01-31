import "@uiw/react-md-editor/markdown-editor.css";
import "@uiw/react-markdown-preview/markdown.css";
import React, { useState, useEffect } from 'react';
import dynamic from 'next/dynamic';
import { useRouter } from "next/router";
import { Button, Container, Dropdown, DropdownHeader, Grid, Header, Icon, Input, Segment } from 'semantic-ui-react';
import rehypeSanitize from "rehype-sanitize";
import { getCodeString } from 'rehype-rewrite';
import katex from 'katex';
import 'katex/dist/katex.css';
import { toast } from "react-hot-toast";
import { Helmet } from "react-helmet";
import { request } from '@/utils/network';
import { AddRequest } from "@/utils/types";
import { BACKEND_URL } from "@/constants/string";

const MDEditor = dynamic(() => import('@uiw/react-md-editor'), {
  ssr: false,
  loading: () => <Segment style={{ height: '500px', marginTop: '10px' }} placeholder loading />
});

type expireTimeType = number | string;

const expireOptions = [
  { key: "1h", text: "1 Hour", value: 3600 },
  { key: "24h", text: "24 Hours", value: 86400 },
  { key: "7d", text: "7 Days", value: 604800 },
  { key: "never", text: "Never", value: 0 },
  { key: "custom", text: "Custom", value: "custom" },
];

const Home: React.FC = () => {
  const [markdownContent, setMarkdownContent] = useState<string | undefined>('');
  const [title, setTitle] = useState<string>('');
  const [expireTime, setExpireTime] = useState<expireTimeType>(3600);
  const [customExpire, setCustomExpire] = useState("");
  const [previewMode, setPreviewMode] = useState<"edit" | "live">("live");
  const [loading, setLoading] = useState<boolean>(false);

  const router = useRouter();

  useEffect(() => {
    const handleResize = () => {
      setPreviewMode(window.innerWidth < 768 ? "edit" : "live");
    };

    handleResize();
    window.addEventListener("resize", handleResize);

    return () => {
      window.removeEventListener("resize", handleResize);
    };
  }, []);

  const handleConfirm = async () => {
    if (loading) return;
    setLoading(true);

    let invalid = false;
    if (!title || title.length === 0) {
      toast.error("`Title` field is required!");
      invalid = true;
    }
    if (!markdownContent || markdownContent.length === 0) {
      toast.error("`Content` field is required!");
      invalid = true;
    }
    if (expireTime === "custom" && (!customExpire || customExpire.length === 0)) {
      toast.error("You need to specify a expiration time!");
      invalid = true;
    }
    if (expireTime === "custom" && !(Number(customExpire) > 0 && Number(customExpire) < 604800)) {
      toast.error("Expiration must be between 1 and 604800 secs.");
      invalid = true;
    }

    if (invalid) {
      setLoading(false);
      return;
    }
    console.log(title, markdownContent, expireTime, customExpire);

    const req: AddRequest = {
      title,
      content: markdownContent || "",
      expiration: expireTime === "custom" ? Number(customExpire) : Number(expireTime)
    };

    await request(`${BACKEND_URL}/add`, "POST", req)
      .then((data) => {
        setLoading(false);
        router.push(`/${data.key}`);
      })
      .catch((err) => {
        console.error(err);
        toast.error("Failed to generate link!");
      })
      .finally(() => {
        setLoading(false);
      });
  };

  const showHelp = () => { console.log("Help") }

  const showAbout = () => { }

  return (
    <Container style={{ padding: '1rem', height: '100vh', textAlign: 'center' }}>
      <Helmet>
        <title>Home - DPB</title>
      </Helmet>
      <Header
        as="h1"
        style={{
          fontSize: "3rem",
          textAlign: "center",
          marginTop: "0.5rem",
          marginBottom: "0.1rem"
        }}
      >
        Welcome to DPB
      </Header>
      <p style={{ fontSize: '1.2rem', marginBottom: '-1rem' }}>
        Share text online with Markdown and KaTeX support.
      </p>
      <Grid centered stackable>
        <Grid.Column mobile={16} tablet={16} computer={16}>
          <div style={{ textAlign: 'right' }}>
            <a style={{ cursor: "pointer" }} onClick={showHelp}>
              <Icon name="help" size="small" />Help
            </a> · <a style={{ cursor: "pointer" }} onClick={showAbout}>
              <Icon name="info circle" size="small" />About
            </a>
          </div>
          <MDEditor
            value={markdownContent}
            onChange={setMarkdownContent}
            textareaProps={{
              placeholder: "Type Markdown text..."
            }}
            style={{ marginTop: '10px' }}
            height={500}
            minHeight={450}
            maxHeight={1200}
            preview={previewMode}
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
          />
        </Grid.Column>
        <Grid.Row columns={2} style={{ padding: 0 }} divided>
          <Grid.Column mobile={16} tablet={8} computer={9}>
            <Input
              placeholder="Title"
              value={title}
              onChange={(e) => setTitle(e.target.value)}
              style={{ width: '100%' }}
              disabled={loading}
            />
          </Grid.Column>
          <Grid.Column mobile={16} tablet={8} computer={7} style={{ display: 'flex' }}>
            <Dropdown
              icon="hourglass end"
              floating
              labeled
              button
              className='icon'
              header={<DropdownHeader icon="time" content="Expire Time" />}
              options={expireOptions}
              value={expireTime}
              onChange={(e, { value }) => {
                setExpireTime(value as expireTimeType)
              }}
              style={{ flex: 0.9, marginRight: '5px', textAlign: 'center' }}
              disabled={loading}
            />

            {expireTime === "custom" && (
              <Input
                type="number"
                placeholder="Secs"
                value={customExpire}
                onChange={(e) => setCustomExpire(e.target.value)}
                style={{ width: '25%', marginRight: '5px' }}
                disabled={loading}
              />
            )}
            <Button
              primary
              icon labelPosition='left'
              style={{ flex: 1.2, textAlign: 'center' }}
              onClick={handleConfirm}
              loading={loading}
            >
              <Icon name="send" />
              Publish
            </Button>
          </Grid.Column>
        </Grid.Row>
      </Grid>
    </Container>
  );
};

export default Home;
