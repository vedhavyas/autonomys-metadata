import Image from 'next/image';
import React from "react";

interface PolkadotVaultProps {
    theme: string;
}

const PolkadotVault: React.FC<PolkadotVaultProps> = ({ theme }) => {
    return (
        <div className="text-center mt-4" style={{ padding: 20, margin: '0 auto', maxWidth: '500px', width: '100%' }}>
            <a href="https://vault.novasama.io/" target="_blank" rel="noreferrer" style={{ display: 'block' }}>
                <Image
                    src={theme === "dark" ? "https://raw.githubusercontent.com/novasamatech/parity-signer/refs/heads/master/docs/src/res/logo-white.svg" : "https://raw.githubusercontent.com/novasamatech/parity-signer/refs/heads/master/docs/src/res/logo-black.svg"}
                    alt="Polkadot Logo"
                    layout="responsive"
                    width={500}
                    height={100}
                />
            </a>
        </div>
    );
};

export default PolkadotVault;