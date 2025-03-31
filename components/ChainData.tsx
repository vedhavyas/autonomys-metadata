import React from "react";
import { Container, Row, Col, Card, Image } from "react-bootstrap";

interface ChainData {
  title: string;
  genesisHash: string;
  unit: string;
  liveMetaVersion: number;
  testnet: boolean;
  icon: string;
}

interface ChainDataCardProps {
  chainData: ChainData;
}

const shortenHash = (hash: string, startLength = 12, endLength = 10) => {
  return `${hash.slice(0, startLength)}...${hash.slice(-endLength)}`;
};

const ChainDataCard: React.FC<ChainDataCardProps> = ({ chainData }) => {
  return (
      <Container>
        <Row className="justify-content-center">
          <Col xs={12} sm={10} md={8} lg={6}>
            <Card className="m-2 p-2">
              <Card.Body className="d-flex flex-column flex-md-row align-items-center">
                <div className="mb-3 mb-md-0">
                  <Image
                      src={chainData.icon}
                      width={100}
                      height={100}
                      rounded
                      alt={`${chainData.title} logo`}
                      className="d-none d-md-block"
                  />
                  <Image
                      src={chainData.icon}
                      width={60}
                      height={60}
                      rounded
                      alt={`${chainData.title} logo`}
                      className="d-block d-md-none"
                  />
                </div>
                <div style={{ marginLeft: "50px" }}>
                  <p><strong>Genesis Hash: </strong>{shortenHash(chainData.genesisHash)}</p>
                  <p><strong>Token: </strong>{chainData.unit}</p>
                  <p><strong>Latest Spec Version: </strong>{chainData.liveMetaVersion}</p>
                  <p><strong>Network: </strong>{chainData.testnet ? "Testnet" : "Mainnet"}</p>
                </div>
              </Card.Body>
            </Card>
          </Col>
        </Row>
      </Container>
  );
};

export default ChainDataCard;