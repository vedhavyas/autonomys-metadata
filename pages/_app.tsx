import "bootstrap/dist/css/bootstrap.min.css";
import { AppProps } from "next/app";
import Head from "next/head";
import '../styles/global.css';

export default function App({ Component, pageProps }: AppProps) {
  return (
    <>
      <Head>
          <link rel="icon" type="image/png" sizes="16x16" href={`${process.env.NODE_ENV === 'production' ? '/autonomys-metadata' : ''}/favicon.png`} />
      </Head>
      <Component {...pageProps} />
    </>
  );
}