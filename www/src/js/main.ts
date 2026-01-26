import '../styles/style.css';

import initEngine, {
  SomeEnum,
  SomeStructure,
  string_return_function,
  error_throw_function,
  classic_alert,
} from 'engine-on-rust';

const engineModule = await initEngine();

let elementsCount = 42;

const struct = new SomeStructure(elementsCount);

let thirdElementDiv = document.getElementById('third-element');
if (thirdElementDiv) {
  thirdElementDiv.innerText =
    struct.get_element(2) == SomeEnum.Var1 ? 'Var1' : 'Var2';
}

let memoryDiv = document.getElementById('memory-data');
let elementsMemory = new Uint8Array(
  engineModule.memory.buffer,
  struct.elements_ptr(),
  elementsCount,
);
if (memoryDiv) {
  memoryDiv.innerText = elementsMemory.join('');
}

console.warn(string_return_function('my string'));

classic_alert();

try {
  error_throw_function();
} catch (e) {
  console.error(e);
}
