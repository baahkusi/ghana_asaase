import { ethereum } from "@graphprotocol/graph-ts";
import {
  ChangedLandOwner as ChangedLandOwnerEvent,
  NewLandOwner as NewLandOwnerEvent,
} from "../generated/GhanaAsaase/GhanaAsaase";
import { Owner, Land, LandHistory } from "../generated/schema";

export function handleChangedLandOwner(event: ChangedLandOwnerEvent): void {
  const land = Land.load(event.params.landAddress);
  if (!land) {
    return;
  }
  const oldOwner = Owner.load(event.params.oldOwner);
  if (!oldOwner) {
    return;
  }
  let newOwner = Owner.load(event.params.newOwner);
  if (!newOwner) {
    newOwner = new Owner(event.params.newOwner);
    newOwner.save();
  }
  land.owner = newOwner.id;
  land.save();
  saveHistory(event, land);
}

export function handleNewLandOwner(event: NewLandOwnerEvent): void {
  const land = new Land(event.params.landAddress);
  const owner = new Owner(event.params.owner);
  owner.save();
  land.owner = owner.id;
  land.save();
  saveHistory(event, land);
}

function saveHistory(
  event: ethereum.Event,
  land: Land
): void {
  const landHistory = new LandHistory(
    land.id.concat(land.owner).concat(event.transaction.hash.toHexString())
  );
  landHistory.blockNumber = event.block.number;
  landHistory.blockTimestamp = event.block.timestamp;
  landHistory.land = land.id;
  landHistory.owner = land.owner;
  landHistory.transactionHash = event.transaction.hash;
  landHistory.save();
}
