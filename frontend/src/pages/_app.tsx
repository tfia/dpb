import 'semantic-ui-css/semantic.min.css'
import type { AppProps } from "next/app";
import { Toaster } from 'react-hot-toast';

export default function App({ Component, pageProps }: AppProps) {
  return (
    <>
      <Component {...pageProps} />
      <Toaster position="top-right" reverseOrder={false} />
    </>
  );
}
