import React, { useEffect, useState } from "react";
import DisplayQr from "../components/DisplayQr";
import ChainDataCard from "../components/ChainData";
import { Dropdown, ButtonGroup, Button, Row, Col } from "react-bootstrap";
import PolkadotVault from "../components/PolkadotVault";

interface SpecData {
  path: string;
}

interface ChainData {
  title: string;
  icon: string;
  latestMetadata: string;
  specsQr: SpecData;
  color: string;
  genesisHash: string;
  unit: string;
  liveMetaVersion: number;
  testnet: boolean;
}

interface DataMap {
  [key: string]: ChainData;
}

function toTitleCase(str: string) {
  return str
    .toLowerCase()
    .split(" ")
    .map((word) => word.charAt(0).toUpperCase() + word.slice(1))
    .join(" ");
}

async function loadData() {
  const chainDataResponse = await fetch(`${process.env.NODE_ENV === 'production' ? '/autonomys-metadata' : ''}/data.json`);
  return chainDataResponse.json();
}

const HomePage = () => {
  const [theme, setTheme] = useState("light");
  const [data, setData] = useState<DataMap | null>(null);
  const [activeButton, setActiveButton] = useState<string>("");
  const [isQrLoaded, setIsQrLoaded] = useState<boolean>(false);

  useEffect(() => {
    const systemTheme = window.matchMedia("(prefers-color-scheme: dark)").matches ? "dark" : "light";
    setTheme(systemTheme);
  }, []);

  useEffect(() => {
    document.body.classList.remove("light");
    document.body.classList.remove("dark");
    document.body.classList.add(theme);
  }, [theme]);

  useEffect(() => {
    if(data && activeButton !== "") {
      setIsQrLoaded(false);
    }
  }, [data, activeButton]);

  useEffect(() => {
    loadData().then((chainData: DataMap) =>{
      setData(chainData);
      setActiveButton(Object.values(chainData)[0].title);
    })
  }, []);


  if (!data || activeButton === "") return "Loading...";
  const activeChainData = data[activeButton.replaceAll(" ", "_")];

  return (
    <div className="container py-5">
      <Row className="align-items-center justify-content-between">
        <Col>
          <h1>Autonomys Metadata</h1>
        </Col>
        <Col xs="auto">
          <Dropdown as={ButtonGroup}>
            <ButtonGroup className="mt-2 ml-2 themeButtonGroup">
              <Button style={{
                backgroundColor: theme === "light" ? activeChainData.color : "grey",
                color: "white"
              }} onClick={() => setTheme("light")}>
                Light
              </Button>
              <Button style={{ backgroundColor: theme === "dark" ? activeChainData.color : "grey", color: "white" }}
                      onClick={() => setTheme("dark")}>
                Dark
              </Button>
            </ButtonGroup>
          </Dropdown>
        </Col>
      </Row>
      <div>
        <ButtonGroup>
          {Object.values(data).map((chainData: ChainData) => (
            <Button
              key={chainData.title}
              onClick={() => {setActiveButton(chainData.title); setIsQrLoaded(false)}}
              style={{
                backgroundColor: chainData.title === activeButton ? chainData.color : "grey",
                borderColor: chainData.title === activeButton ? chainData.color : "grey",
                border: "0px solid"
              }}
              variant={chainData.title === activeButton ? "primary" : "secondary"}
              className="m-2 rounded-pill"
            >
              {toTitleCase(chainData.title)}
            </Button>
          ))}
        </ButtonGroup>
      </div>

      <ChainDataCard chainData={activeChainData} />

      <div className="container">
        {activeChainData &&
          <DisplayQr
            key={activeButton}
            latestMetadata={activeChainData.latestMetadata}
            specsQrPath={activeChainData.specsQr.path}
            color={activeChainData.color}
            onLoad={() => setIsQrLoaded(true)}
          />}
      </div>

      {isQrLoaded && <PolkadotVault theme={theme} />}
    </div>
  );
};

export default HomePage;