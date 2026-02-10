import React, { useState, useEffect } from 'react';
import { Paper, Typography, Box, Grid, Card, CardContent, Avatar, CircularProgress, Alert } from '@mui/material';

interface App {
  appid: string;
  name: string;
  icon: string;
  description: string;
}

interface ApiResponse {
  status: string;
  data: App[];
  count: number;
}

const AppLibrary: React.FC = () => {
  const [apps, setApps] = useState<App[]>([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    const fetchApps = async () => {
      try {
        const response = await fetch('http://127.0.0.1:3000/apps-metadata');
        if (!response.ok) {
          throw new Error(`HTTP error! status: ${response.status}`);
        }
        const data: ApiResponse = await response.json();
        if (data.status === 'success' && data.data) {
          setApps(data.data);
        } else {
          throw new Error('Invalid response format');
        }
      } catch (err) {
        setError(err instanceof Error ? err.message : '获取应用列表失败');
      } finally {
        setLoading(false);
      }
    };

    fetchApps();
  }, []);

  if (loading) {
    return (
      <Box display="flex" justifyContent="center" alignItems="center" minHeight="60vh">
        <CircularProgress />
      </Box>
    );
  }

  if (error) {
    return (
      <Box display="flex" justifyContent="center" alignItems="center" minHeight="60vh">
        <Alert severity="error" sx={{ maxWidth: 400 }}>
          {error}
        </Alert>
      </Box>
    );
  }

  return (
    <Box>
      <Grid container spacing={3}>
        {apps.map((app) => (
          <Grid item xs={12} sm={6} md={4} key={app.appid}>
            <Card sx={{ height: '100%', display: 'flex', flexDirection: 'column' }}>
              <CardContent>
                <Box display="flex" alignItems="center" mb={2}>
                  <Avatar
                    src={app.icon}
                    alt={app.name}
                    sx={{ width: 48, height: 48, mr: 2, flexShrink: 0 }}
                  />
                  <Typography variant="h6" component="h2" noWrap>
                    {app.name}
                  </Typography>
                </Box>
                <Typography variant="body2" color="text.secondary">
                  {app.description}
                </Typography>
              </CardContent>
            </Card>
          </Grid>
        ))}
      </Grid>
    </Box>
  );
};

export default AppLibrary;