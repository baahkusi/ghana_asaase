# About
This repo was made for this tutorial for designing and implementing a smart contract. Below is the problem statement used.

## Problem

Ghana has two important systems in place;
1. Ghana Card - a unique identification card for every Ghanaian. It is an identification system that assigns every Ghanaian a unique code.
2. Ghana Post GPS - a system that assigns a unique code (address) to every 100² m plot of land in Ghana. An identification system for lands.

We want to develop a simple land registration system on the blockchain where we assign a land address to the owner. Here's a quick list of features it should have.

- Allow an admin to assign the ownership of a unit of land to a Ghanaian.
- Allow an admin to change the ownership of a unit of land.
- When the system is queried with the ID of a Ghanaian, it should return all the lands owned by them.
- When the system is queried with the address of a unit of land it should return the Ghanaian who owns the land.