import { useEffect, useState } from 'react';
import { invoke } from '@tauri-apps/api/tauri';
import { hide, show } from '@tauri-apps/api/app';
import { Button, Input, Card } from 'antd';
import { Fibonacci } from '@/components/Fibonacci';
import { WasmTst } from '@/components/WasmTst';
import './App.css';
import '@/styles/index.less';

function App() {
  const [greetMsg, setGreetMsg] = useState('');
  const [name, setName] = useState('');

  async function greet() {
    // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
    setGreetMsg(await invoke('greet', { name }));
  }

  const handleRustIter = async () => {
    await invoke('tst_rust_func');
  };

  const handleIter = (): void => {
    console.time('iter');
    const arr = [];
    for (let i = 0; i < 1000000; i++) {
      arr.push(i);
    }
    console.timeEnd('iter');
  };

  const handleHide = () => {
    hide();
  };

  const [num, setNum] = useState<number>(0);

  return (
    <>
      <div className="container">
        <Card hoverable style={{ width: 600 }}>
          <Input
            onChange={(e) => setName(e.currentTarget.value)}
            placeholder="Enter a name..."
          />
          <Button onClick={greet}>Greet</Button>
        </Card>
        <div>{greetMsg}</div>
        <Button onClick={handleHide}>hide</Button>
        <Button onClick={handleRustIter}>rust迭代</Button>
        <Button onClick={handleIter}>js迭代</Button>
        <Fibonacci setNum={setNum} />
        <WasmTst num={num} />
      </div>
    </>
  );
}

export default App;
