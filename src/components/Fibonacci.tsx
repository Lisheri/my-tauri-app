import { useState } from 'react';
import { Card, Input, Button } from 'antd';
import { invoke } from '@tauri-apps/api/tauri';

export const Fibonacci: React.FC<{
  setNum: React.Dispatch<React.SetStateAction<number>>
}> = (props) => {
  const [fibNum, setFibNum] = useState<number>(0);
  const [fibRes, setFibRes] = useState<string>('');
  const [fibResR, setFibResR] = useState<string>('');
  const fibonacci = (n: number) => {
    let pre1 = BigInt(1);
    let pre2 = BigInt(1);
    let current = BigInt(1);
    if (n < 3) return current.toString();
    for (let i = 2; i < n; i++) {
      pre1 = pre2;
      pre2 = current;
      current = pre1 + pre2;
    }
    return current.toString();
  };

  const calcRust = async () => {
    console.time('fibonacci');
    const res: string = await invoke('fibonacci_func', { n: fibNum });
    console.timeEnd('fibonacci');
    setFibResR(res);
  }

  const handleFibNumChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    setFibNum(parseInt(e.currentTarget.value) || 0);
    props.setNum(parseInt(e.currentTarget.value) || 0)
  };
  const handleCalc = () => {
    console.time('fibonacci');
    const fibonacciRes = fibonacci(fibNum);
    console.timeEnd('fibonacci');
    setFibRes(fibonacciRes);
  };
  return (
    <Card hoverable style={{ width: 600 }}>
      <Input onChange={handleFibNumChange} placeholder="请输入原始值" />
      <div>当前值为: {fibRes}</div>
      <Button onClick={handleCalc}>计算</Button>
      <div>Rust计算结果: {fibResR}</div>
      <Button onClick={calcRust}>调用Rust计算</Button>
    </Card>
  );
};
