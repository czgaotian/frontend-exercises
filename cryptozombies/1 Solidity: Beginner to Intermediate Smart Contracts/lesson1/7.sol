pragma solidity >=0.5.0 <0.6.0;

contract ZombieFactory {

    uint dnaDigits = 16;
    uint dnaModulus = 10 ** dnaDigits;

    struct Zombie {
        string name;
        uint dna;
    }

    Zombie[] public zombies;

    // The 'memory' keyword is required for all reference types such as arrays, structs, mappings, and strings.
    function createZombie (string memory _name, uint _dna) public {
        // start here
    }

}
