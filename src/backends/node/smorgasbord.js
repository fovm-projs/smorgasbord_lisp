// Functions

// Arithmetics
const add = (first, second) => first + second;
const sub = (first, second) => first - second;
const mul = (first, second) => first * second;
const div = (first, second) => first / second;
const mod = (first, second) => first % second;
const eq = (first, second) => first === second;
const neq = (first, second) => first !== second;
const lt = (first, second) => first < second;
const le = (first, second) => first <= second;
const gt = (first, second) => first > second;
const ge = (first, second) => first >= second;

// Bitwise operations
const band = (first, second) => first & second;
const bor = (first, second) => first | second;
const bxor = (first, second) => first ^ second;
const bsl = (first, second) => first << second;
const bsr = (first, second) => first >> second;
const bnot = (value) => ~value;

// Logical
const land = (first, second) => first && second;
const lor = (first, second) => first || second;
const lnot = (value) => !value;

// System calls
// Input/Output
function write(descriptor_no, value) {
  switch (descriptor_no) {
    case 0:
      console.log(value);
      break;
    default:
      console.log("nah buddy you aint legal here");
  }
}

// Environment
function get_environment(key) {
  const n = Number(key);

  if (Number.isInteger(n) && key.trim() !== "") {
    return process.argv[n];
  }

  return process.env[key];
}

function set_environment(key, value) {
  process.env[key] = value;
}
