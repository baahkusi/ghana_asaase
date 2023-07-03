// SPDX-License-Identifier: MIT
pragma solidity ^0.8.9;

contract GhanaAsaase {
    address admin;
    // stores owner for each address
    mapping(string => string) public landOwner;
    event NewLandOwner(string landAddress, string owner);
    event ChangedLandOwner(string landAddress, string oldOwner, string newOwner);
    error ExistingLandOwner(string);
    error NoLandOwner();
    constructor(address admin_) {
        admin = admin_;
    }
    modifier onlyOwner {
        require( msg.sender == admin, "Only admin can make a call");
        _;
    }
    modifier noLandOwner(string calldata landAddress) {
        if (bytes(landOwner[landAddress]).length > 0) {
            revert ExistingLandOwner(landOwner[landAddress]);
        }
        _;
    }
    modifier hasLandOwner(string calldata landAddress) {
        if (bytes(landOwner[landAddress]).length == 0) {
            revert NoLandOwner();
        }
        _;
    }
    /**
    * Assign a land to an owner
    * can only be called by admin
     */
    function assignOwnership(string calldata landAddress, string calldata owner) public onlyOwner noLandOwner(landAddress) returns(bool) {        
        landOwner[landAddress] = owner;
        emit NewLandOwner(landAddress, owner);
        return true;
    }
    /**
    * change owner of land to new owner
    * can only be called by admin
     */
    function changeOwnership(string calldata landAddress, string calldata newOwner) public onlyOwner hasLandOwner(landAddress) returns(bool) {
        emit ChangedLandOwner(landAddress, landOwner[landAddress], newOwner);
        landOwner[landAddress] = newOwner;
        return true;
    }
}
