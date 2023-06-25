import { useEffect } from 'react';
import init, { greet, fibonacci } from '@mywasm/foo';
import { Button } from 'antd';
export const WasmTst: React.FC<{
  num: number
}> = (props) => {
  const { num } = props;
  useEffect(() => {
    // 初始化, 加载 wasm
    init();
  }, []);
  const accu = () => {
    console.time('wasm');
    const res = fibonacci(num);
    console.timeEnd('wasm');
    console.info(res, 'resres');
  }

  return (
    <div>
      <Button onClick={() => greet()}>click me</Button>
      <Button onClick={accu}>wasm计算</Button>
    </div>
  );
};
