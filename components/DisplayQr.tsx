import React, { useState } from "react";
import Image from "react-bootstrap/Image";
import Card from "react-bootstrap/Card";
import ButtonGroup from "react-bootstrap/ButtonGroup";
import Button from "react-bootstrap/Button";

interface ImagesDisplayProps {
  latestMetadata: string;
  specsQrPath: string;
  color: string;
}

const DisplayQr: React.FC<ImagesDisplayProps> = ({ latestMetadata, specsQrPath, color }) => {
  const [activeImage, setActiveImage] = useState(specsQrPath);

  return (
    <div className="d-flex flex-column align-items-center">
      <ButtonGroup className="mb-2">
        <Button
          onClick={() => setActiveImage(specsQrPath)}
          style={{
            backgroundColor: activeImage === specsQrPath ? color : "grey",
            borderColor: activeImage === specsQrPath ? color : "grey",
            border: "0px solid"
          }}
          className="m-2 rounded-pill"
        >
          Chain spec
        </Button>
        <Button
          onClick={() => setActiveImage(latestMetadata)}
          style={{
            backgroundColor: activeImage === latestMetadata ? color : "grey",
            borderColor: activeImage === specsQrPath ? color : "grey",
            border: "0px solid"
          }}
          className="m-2 rounded-pill"
        >
          Metadata
        </Button>
      </ButtonGroup>
      <Card className="m-2 p-0">
        <Card.Body className="text-center">
          <Image src={activeImage} fluid className="mx-auto d-block" alt={`${activeImage} logo`} />
        </Card.Body>
      </Card>
    </div>
  );
};

export default DisplayQr;