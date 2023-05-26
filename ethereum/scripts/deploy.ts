import { ethers } from "hardhat";

async function main() {

  const GhanaAsaase = await ethers.getContractFactory("GhanaAsaase");
  const admins = [""];
  const ghanaAsaase = await GhanaAsaase.deploy(admins);

  await ghanaAsaase.deployed();

  console.log(
    `Contract deployed to ${ghanaAsaase.address}`
  );
}

// We recommend this pattern to be able to use async/await everywhere
// and properly handle errors.
main().catch((error) => {
  console.error(error);
  process.exitCode = 1;
});
