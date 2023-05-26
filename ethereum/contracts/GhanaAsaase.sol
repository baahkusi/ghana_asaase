// SPDX-License-Identifier: MIT
pragma solidity ^0.8.9;

contract GhanaAsaase {
    address admin;
    // stores owner for each address
    mapping(string => address) public landOwner;
    event NewLandOwner(string landAddress, address owner);
    event ChangedLandOwner(
        string landAddress,
        address oldOwner,
        address newOwner
    );
    error WrongLandOwner(address);
    error ExistingLandOwner(address);
    error NoLandOwner();

    constructor(address admin_) {
        admin = admin_;
    }

    modifier onlyOwner() {
        require(msg.sender == admin, "Only admin can make a call");
        _;
    }

    /**
     * Assign a land to an owner
     * can only be called by admin
     */
    function assignOwnership(
        string calldata landAddress,
        address owner
    ) public returns (bool) {
        if (landOwner[landAddress] != address(0)) {
            revert ExistingLandOwner(landOwner[landAddress]);
        }
        landOwner[landAddress] = owner;
        emit NewLandOwner(landAddress, owner);
        return true;
    }

    /**
     * change owner of land to new owner
     * can only be called by admin
     */
    function changeOwnership(
        string calldata landAddress,
        address oldOwner,
        address newOwner
    ) public returns (bool) {
        if (landOwner[landAddress] == address(0)) {
            revert NoLandOwner();
        }
        if (landOwner[landAddress] != oldOwner) {
            revert WrongLandOwner(landOwner[landAddress]);
        }
        landOwner[landAddress] = newOwner;
        emit ChangedLandOwner(landAddress, oldOwner, newOwner);
        return true;
    }
}
