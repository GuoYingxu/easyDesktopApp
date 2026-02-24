// 日志配置
export interface LogConfig {
  max_file_size: number | null;
  keep_files: number;
  level: string;
  to_stdout: boolean;
}