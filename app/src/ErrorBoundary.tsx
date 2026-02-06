import { Component, ErrorInfo, ReactNode } from "react";

interface Props {
  children: ReactNode;
}

interface State {
  hasError: boolean;
  error: Error | null;
}

export class ErrorBoundary extends Component<Props, State> {
  constructor(props: Props) {
    super(props);
    this.state = { hasError: false, error: null };
  }

  static getDerivedStateFromError(error: Error): State {
    return { hasError: true, error };
  }

  componentDidCatch(error: Error, errorInfo: ErrorInfo) {
    console.error("Uncaught error:", error, errorInfo);
  }

  render() {
    if (this.state.hasError) {
      return (
        <main className="container">
          <h1>Something went wrong</h1>
          <p style={{ color: "#ff6b6b", margin: "1rem 0" }}>
            {this.state.error?.message}
          </p>
          <button onClick={() => this.setState({ hasError: false, error: null })}>
            Try again
          </button>
        </main>
      );
    }

    return this.props.children;
  }
}
